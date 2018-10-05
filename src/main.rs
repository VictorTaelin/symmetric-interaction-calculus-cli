extern crate clap;
use clap::{Arg, App};

extern crate sic;
use sic::term::*;
use sic::term::Term::*;

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn parse_binary_input(s : &[u8], i : u32) -> Term {
    match if s.len() > 0 { s[0] } else { b' ' } {
        b'0' => {
            let nam = new_name(i+1);
            let app = Term::App{
                fun: Box::new(Var{nam: nam.clone()}),
                arg: Box::new(parse_binary_input(&s[1..], i+1))
            };
            let e_lam = Term::Lam{
                nam: b"-".to_vec(),
                bod: Box::new(app)
            };
            let i_lam = Term::Lam{
                nam: b"-".to_vec(),
                bod: Box::new(e_lam)
            };
            let o_lam = Term::Lam{
                nam: nam,
                bod: Box::new(i_lam)
            };
            o_lam
        },
        b'1' => {
            let nam = new_name(i+1);
            let app = Term::App{
                fun: Box::new(Var{nam: nam.clone()}),
                arg: Box::new(parse_binary_input(&s[1..], i+1))
            };
            let e_lam = Term::Lam{
                nam: b"-".to_vec(),
                bod: Box::new(app)
            };
            let i_lam = Term::Lam{
                nam: nam,
                bod: Box::new(e_lam)
            };
            let o_lam = Term::Lam{
                nam: b"-".to_vec(),
                bod: Box::new(i_lam)
            };
            o_lam
        },
        _ => {
            let nam = new_name(i+1);
            let var = Var{nam: nam.clone()};
            let e_lam = Term::Lam{
                nam: nam,
                bod: Box::new(var)
            };
            let i_lam = Term::Lam{
                nam: b"-".to_vec(),
                bod: Box::new(e_lam)
            };
            let o_lam = Term::Lam{
                nam: b"-".to_vec(),
                bod: Box::new(i_lam)
            };
            o_lam
        }
    }
}

// Can this style be improved?
fn format_binary_output(t : &Term) -> Vec<u8> {
    fn format_binary_output(t : &Term, v : &mut Vec<u8>) {
        match t {
            Term::Lam{nam: ref o_nam, bod: ref o_bod} => {
                match **o_bod {
                    Term::Lam{nam: ref i_nam, bod: ref i_bod} => {
                        match **i_bod {
                            Term::Lam{nam: _, bod: ref e_bod} => {
                                match **e_bod {
                                    Term::App{fun: ref app_fun, arg: ref app_arg} => {
                                        match **app_fun {
                                            Term::Var{nam: ref var_nam} => {
                                                if var_nam == o_nam {
                                                    v.extend_from_slice(b"0");
                                                    format_binary_output(app_arg, v);
                                                } else if var_nam == i_nam {
                                                    v.extend_from_slice(b"1");
                                                    format_binary_output(app_arg, v);
                                                }
                                            },
                                            _ => {}
                                        }
                                    },
                                    _ => {}
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    let mut v : Vec<u8> = Vec::new();
    format_binary_output(t, &mut v);
    v
}

fn main() -> io::Result<()> {
    let matches = App::new("Symmetric Interaction Calculus")
        .version("0.1.0")
        .author("Victor Maia <srvictormaia@gmail.com>")
        .about("Evaluates SIC programs")
        .arg(Arg::with_name("INPUT")
            .short("i")
            .long("input")
            .value_name("INPUT")
            .help("Input term")
            .takes_value(true))
        .arg(Arg::with_name("BINPUT")
            .short("b")
            .long("binput")
            .value_name("BINPUT")
            .help("Input term, encoded as a binary string")
            .takes_value(true))
        .arg(Arg::with_name("BOUTPUT")
            .short("B")
            .long("boutput")
            .value_name("BOUTPUT")
            .help("Decodes output as a binary string")
            .takes_value(false))
        .arg(Arg::with_name("STATS")
            .short("s")
            .long("stats")
            .value_name("STATS")
            .help("Show stats")
            .takes_value(false))
        .arg(Arg::with_name("FILE")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    let file_name = matches.value_of("FILE").unwrap();
    let mut file = File::open(file_name)?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let input : Option<Vec<u8>> = match matches.value_of("BINPUT") {
        Some(bits) => Some(to_string(&parse_binary_input(bits.as_bytes(), 0))),
        None => match matches.value_of("INPUT") {
            Some(bits) => Some(bits.as_bytes().to_vec()),
            None => None
        }
    };

    match input {
        Some(mut input) => {
            code.extend_from_slice(b"\n:main ");
            code.append(&mut input);
        },
        None => {}
    }

    let term = from_string(&code);
    let mut net = to_net(&term);
    let stats = sic::net::reduce(&mut net);
    let norm = from_net(&net);

    let output = if matches.is_present("BOUTPUT") {
        format_binary_output(&norm)
    } else {
        to_string(&norm)
    };

    println!("{}", String::from_utf8_lossy(&output));

    if matches.is_present("STATS") {
        println!("{:?}", stats);
    }

    Ok(())
}

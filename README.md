## Symmetric Interaction Calculus CLI

CLI to evaluate [Symmetric Interaction Calculus](https://github.com/maiavictor/symmetric-interaction-calculus) programs.

```
cargo install symmetric-interaction-calculus-cli
echo ": #x x #y y" >> example.sic
sic example.sic
```

You can also supply a SIC input to the SIC program:

```
echo "/main #input input" >> example.sic
sic example.sic -i "#x x"
```

Or a binary input:

```
echo "/main #input input" >> example.sic
sic example.sic -b "101011"
```

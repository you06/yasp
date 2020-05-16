# YASP

Yet Another SQL Parser.

This parser is designed for standard SQL.

## Usage

Add dependency to your `Cargo.toml`.

```toml
[dependencies]
yasp = { git = "https://github.com/airyworks/yasp" }
```

```rust
use yasp::grammar;

fn main() {
    let exprs = grammar::ExprsParser::new()
        .parse("select * from sakura; select rin, shizuku, * from uta")
        .unwrap();

    for expr in exprs {
        println!("expr: {}", expr);
    }
}
```

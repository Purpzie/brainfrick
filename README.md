# Brainfrick
[![Build](https://github.com/Purpzie/brainfrick/workflows/CI/badge.svg)](https://github.com/Purpzie/brainfrick/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/brainfrick.svg)](https://crates.io/crates/brainfrick)

A brainfuck interpreter with helpful errors. [See the documentation.](https://docs.rs/brainfrick)

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
brainfrick = "1.1.2"
```

### Example
```rust
use brainfrick::Brainfuck;

let purpzie_sucks = Brainfuck::execute("
    ++++++++[>++++++++++<-]>.<++[>++++++++++<-]+++[>+++++<-]>+
    +.---.--.++++++++++.<++[>----------<-]>+++.----.<+++++++[>
    ----------<-]>+.<++++++++[>++++++++++<-]>+++.++.<+++[>----
    --<-]>.++++++++.++++++++.<++++++++[>----------<-]>--.
")?;

assert_eq!(purpzie_sucks, "Purpzie sucks!");
```

---

## License
Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

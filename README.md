# covfefe 

[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![version](https://img.shields.io/crates/v/covfefe.svg)](https://crates.io/crates/covfefe/)
[![Build Status](https://travis-ci.org/Feliix42/covfefe.svg?branch=master)](https://travis-ci.org/Feliix42/covfefe)

This is a small crate to end tweets or sentences with Style. Inspired by [rgbkrk](https://github.com/rgbkrk/covfefe) and the [45th president](https://archive.is/f7UL3) of the United States of America.

## Usage

Just add this crate to your dependencies:

```
[dependencies]
covfefe = "0.1"
```

And then use it in your application:

```rust
use covfefe::Covfefe;

fn main() {
    println!("{}", "Despite the constant negative press".covfefe());
}
```

## Credits

All credits go to [rgbkrk](https://github.com/rgbkrk/covfefe), who had the original idea and implemented it in node.

## License

This work is published under the MIT License.

# `aipn-rust` - All Assigned Internet Protocol Numbers in Rust

[![Crates.io](https://img.shields.io/crates/v/aipn)](https://crates.io/crates/aipn)
[![Docs.rs](https://docs.rs/aipn/badge.svg)](https://docs.rs/aipn)

## Introduction

This crate/repository provides a list of all known Assigned Internet Protocol Numbers as per the [IANA Protocol Numbers](https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml).
It was created because I required a list of all protocol numbers for a project and could not find a rust crate that provided this information. Including this manually just for a single project, seemed like a waste of time. Therefore, to avoid repeating this process, I decided to create this crate.

## Usage

To use this crate, add the following to your `Cargo.toml`:

```toml
[dependencies]
aipn = "0.1.2"
```

Then, you can use the crate as follows:

```rust
use aipn::AIPN;

let protocol = AIPN::TCP;
println!("Protocol type: {:?}", protocol);

match protocol {
    AIPN::TCP => {
        // Adjust code execution according to the protocol
    },
    _ => println!("Unknown protocol"),
}

let some_protocol_value=17;
let protocol=AIPN::from_u8(some_protocol_value);
println!("Protocol type: {:?}", protocol);
```

## License
This crate is licensed under the MIT license. See the [LICENSE](LICENSE) file for more details.

## Contribution
If you would like to contribute to this crate, feel free to open a pull request or an issue. I am always open to suggestions and improvements.
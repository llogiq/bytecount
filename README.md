# bytecount

Counting bytes really fast

[![Build Status](https://travis-ci.org/llogiq/bytecount.svg?branch=master)](https://travis-ci.org/llogiq/bytecount)
[![Current Version](http://meritbadge.herokuapp.com/bytecount)](https://crates.io/crates/bytecount)
[![License: Apache 2.0/MIT](https://img.shields.io/crates/l/bytecount.svg)](#License)

This uses the "hyperscreamingcount" algorithm by Joshua Landau to count bytes
faster than anything else. The [newlinebench](/llogiq/newlinebench) repository
has further benchmarks.

To use bytecount in your crate, if you have
[cargo-edit](/killercup/cargo-edit), just type `cargo add bytecount` in a
terminal with the crate root as the current path. Otherwise you can manually
edit your `Cargo.toml` to add `bytecount = 0.0.1` to your `[dependencies]`
section.

In your crate root (`lib.rs` or `main.rs`, depending on if you are writing a
library or application), add `extern crate bytecount;`. Now you can simply use
`bytecount::count` as follows:

```Rust
extern crate bytecount;

fn main() {
    let mytext = "some potentially large text, perhaps read from disk?";
    let spaces = bytecount::count(mytext.as_bytes(), b' ');
    ..
}
```

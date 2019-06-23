# bugsalot

<!-- [![Build Status](https://travis-ci.org/MaulingMonkey/bugsalot.svg)](https://travis-ci.org/MaulingMonkey/bugsalot) -->
![GitHub](https://img.shields.io/github/stars/MaulingMonkey/bugsalot.svg?label=GitHub&style=social)
[![Build status](https://ci.appveyor.com/api/projects/status/nyvlrelifcyjc1l1?svg=true)](https://ci.appveyor.com/project/MaulingMonkey/bugsalot)
[![Crates.io](https://img.shields.io/crates/v/bugsalot.svg)](https://crates.io/crates/bugsalot)
![unsafe: yes](https://img.shields.io/badge/unsafe-yes-yellow.svg)
[![Open issues](https://img.shields.io/github/issues-raw/MaulingMonkey/bugsalot.svg)](https://github.com/MaulingMonkey/bugsalot/issues)
[![License](https://img.shields.io/crates/l/bugsalot.svg)](https://github.com/MaulingMonkey/bugsalot)
[![Docs](https://docs.rs/bugsalot/badge.svg)](https://docs.rs/bugsalot/)

This crate provides macros and methods for bug wrangling.  Specifically, I want all the advantages of crashing (bug
reports, ease of debugging, etc.) with none of the drawbacks (lost progress, pissed off gamers, etc).  Rust's error
handling mechanisms (`Try`, `?`, `Result`s, etc.) are great, but leave something to be desired when it comes to actual
bugs.  Similarly, Rust's `panic!`, `.unwrap()`, `.expect()`, etc. are decent when it comes to giving context for bugs,
but less great for writing stable software.  This crate will attempt to bridge the gap.

## Platforms

| Platform  | Breakpoints | Debugger  | CI |
| --------- | ----------- | --------- | -- |
| Windows   | ![Supported](https://img.shields.io/badge/-supported-green.svg) | ![Supported](https://img.shields.io/badge/-supported-green.svg) | ![Tests](https://img.shields.io/badge/-tests-green.svg)   |
| Android   | ![Supported](https://img.shields.io/badge/-supported-green.svg) | ![Supported](https://img.shields.io/badge/-supported-green.svg) | ![No](https://img.shields.io/badge/-no-red.svg)           |
| Linux     | ![Supported](https://img.shields.io/badge/-supported-green.svg) | ![Supported](https://img.shields.io/badge/-supported-green.svg) | ![No](https://img.shields.io/badge/-no-red.svg)           |
| FreeBSD   | ![Untested](https://img.shields.io/badge/-untested-yellow.svg)  | ![Untested](https://img.shields.io/badge/-untested-yellow.svg)  | ![No](https://img.shields.io/badge/-no-red.svg)           |
| NetBSD    | ![Untested](https://img.shields.io/badge/-untested-yellow.svg)  | ![Untested](https://img.shields.io/badge/-untested-yellow.svg)  | ![No](https://img.shields.io/badge/-no-red.svg)           |
| OS X      | ![Untested](https://img.shields.io/badge/-untested-yellow.svg)  | ![Untested](https://img.shields.io/badge/-untested-yellow.svg)  | ![No](https://img.shields.io/badge/-no-red.svg)           |
| iOS       | ![Untested](https://img.shields.io/badge/-untested-yellow.svg)  | ![Untested](https://img.shields.io/badge/-untested-yellow.svg)  | ![No](https://img.shields.io/badge/-no-red.svg)           |
| WASM      | ![Supported](https://img.shields.io/badge/-supported-green.svg) | ![N/A](https://img.shields.io/badge/-N/A-red.svg)               | ![Build](https://img.shields.io/badge/-build-yellow.svg)  |

## Quick Start

Add to your cargo.toml:
```toml
[dependencies]
bugsalot = "0.1"
```

Write your code (see [examples](examples) and [documentation](https://docs.rs/bugsalot/) for more code):
```rs
use bugsalot::debugger;

fn main() {
    let _ = debugger::wait_until_attached(None); // Wait for a debugger to be attached

    loop {
        let a : Option<i32> = Some(42);
        let b : Result<i32, &'static str> = Err("Unavailable");
        let a = expect!(a, "Unable to do something or other", return);
        let b = expect!(b, "Unable to do something or other", break);
        // Debugger will pause on the above line, continuing will break out of the loop
    }

    expect!(true, "Booleans work too");
}
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

<!-- https://doc.rust-lang.org/1.4.0/complement-project-faq.html#why-dual-mit/asl2-license? -->
<!-- https://rust-lang-nursery.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive -->
<!-- https://choosealicense.com/licenses/apache-2.0/ -->
<!-- https://choosealicense.com/licenses/mit/ -->


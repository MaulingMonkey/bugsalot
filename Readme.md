# bugsalot

<!-- [![Build status](https://ci.appveyor.com/api/projects/status/???/master?svg=true)](https://ci.appveyor.com/project/MaulingMonkey/bugsalot) -->
<!-- [![Build Status](https://travis-ci.org/MaulingMonkey/bugsalot.svg)](https://travis-ci.org/MaulingMonkey/bugsalot) -->
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

<!-- https://doc.rust-lang.org/1.4.0/complement-project-faq.html#why-dual-mit/asl2-license? -->
<!-- https://rust-lang-nursery.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive -->
<!-- https://choosealicense.com/licenses/apache-2.0/ -->
<!-- https://choosealicense.com/licenses/mit/ -->


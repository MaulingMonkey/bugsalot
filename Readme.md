# bugsalot

<!-- [![Build status](https://ci.appveyor.com/api/projects/status/nyvlrelifcyjc1l1?svg=true)](https://ci.appveyor.com/project/MaulingMonkey/bugsalot) -->
[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/bugsalot.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/bugsalot)
[![unsafe: yes](https://img.shields.io/github/search/MaulingMonkey/bugsalot/unsafe%2bextension%3Ars?color=yellow&label=unsafe)](https://github.com/MaulingMonkey/bugsalot/search?q=unsafe+extension%3Ars)
[![rust: 1.36.0+](https://img.shields.io/badge/rust-1.36.0%2B-green.svg)](https://gist.github.com/MaulingMonkey/c81a9f18811079f19326dac4daa5a359#minimum-supported-rust-versions-msrv)
[![License](https://img.shields.io/crates/l/bugsalot.svg)](https://github.com/MaulingMonkey/bugsalot)
[![dependency status](https://deps.rs/repo/github/MaulingMonkey/bugsalot/status.svg)](https://deps.rs/repo/github/MaulingMonkey/bugsalot)

This crate provides macros and methods for bug wrangling.  Specifically, I want all the advantages of crashing (bug
reports, ease of debugging, etc.) with none of the drawbacks (lost progress, pissed off gamers, etc).  Rust's error
handling mechanisms (`Try`, `?`, `Result`s, etc.) are great, but leave something to be desired when it comes to actual
bugs.  Similarly, Rust's `panic!`, `.unwrap()`, `.expect()`, etc. are decent when it comes to giving context for bugs,
but less great for writing stable software.  This crate will attempt to bridge the gap.

| Branch | Badges | Notes |
| ------ | ------ | ----- |
| [publish] | [![Crates.io](https://img.shields.io/crates/v/bugsalot.svg)](https://crates.io/crates/bugsalot) [![Docs](https://docs.rs/bugsalot/badge.svg)](https://docs.rs/bugsalot/) | Stable/published version
| [master]  | [![Build Status](https://travis-ci.org/MaulingMonkey/bugsalot.svg)](https://travis-ci.org/MaulingMonkey/bugsalot) [![Open issues](https://img.shields.io/github/issues-raw/MaulingMonkey/bugsalot.svg)](https://github.com/MaulingMonkey/bugsalot/issues) | "Completed" stuff that hasn't been published.
| wip/*     | | "Work In Progress" - incomplete, use at your own risk.
| dead/*    | | Abandoned threads of work

[publish]:      https://github.com/MaulingMonkey/bugsalot/tree/publish
[master]:       https://github.com/MaulingMonkey/bugsalot/tree/master



## Platforms

| Platform  | Breakpoints | Debugger  | CI | Stable | Beta | Nightly |
| --------- | ----------- | --------- | -- | ------ | ---- | ------- |
| Windows   | ![Supported] | ![Supported] | ![Tests] | ![Status](https://travis-matrix-badges.herokuapp.com/repos/MaulingMonkey/bugsalot/branches/wip-travis/4) |
| Android   | ![Supported] | ![Supported] | ![Build] | ![Status](https://travis-matrix-badges.herokuapp.com/repos/MaulingMonkey/bugsalot/branches/wip-travis/7) |
| Linux     | ![Supported] | ![Supported] | ![Tests] | ![Status](https://travis-matrix-badges.herokuapp.com/repos/MaulingMonkey/bugsalot/branches/wip-travis/2) | ![Status](https://travis-matrix-badges.herokuapp.com/repos/MaulingMonkey/bugsalot/branches/wip-travis/3) | ![Status](https://travis-matrix-badges.herokuapp.com/repos/MaulingMonkey/bugsalot/branches/wip-travis/9) |
| (Release) |              |              |          | ![Status](https://travis-matrix-badges.herokuapp.com/repos/MaulingMonkey/bugsalot/branches/wip-travis/1) |
| FreeBSD   | ![Untested]  | ![Untested]  | ![No]    |
| NetBSD    | ![Untested]  | ![Untested]  | ![No]    |
| OS X      | ![Untested]  | ![Untested]  | ![Tests] | ![Status](https://travis-matrix-badges.herokuapp.com/repos/MaulingMonkey/bugsalot/branches/wip-travis/5) |
| iOS       | ![Untested]  | ![Untested]  | ![Build] | ![Status](https://travis-matrix-badges.herokuapp.com/repos/MaulingMonkey/bugsalot/branches/wip-travis/6) |
| WASM      | ![Supported] | ![N/A]       | ![Build] | ![Status](https://travis-matrix-badges.herokuapp.com/repos/MaulingMonkey/bugsalot/branches/wip-travis/8) |

[Supported]:            https://img.shields.io/badge/-supported-green.svg
[Untested]:             https://img.shields.io/badge/-untested-yellow.svg
[N/A]:                  https://img.shields.io/badge/-N/A-red.svg
[Tests]:                https://img.shields.io/badge/-tests-green.svg
[Build]:                https://img.shields.io/badge/-build-yellow.svg
[No]:                   https://img.shields.io/badge/-no-red.svg



## Quick Start

Add **one** of the following bugsalot dependencies to your Cargo.toml:
```toml
[dependencies]
bugsalot = "0.2"                                            # Or...
bugsalot = { version = "0.2", features = ["wasm-bindgen"] } # If using: wasm-pack
bugsalot = { version = "0.2", features = ["stdweb"]       } # If using: cargo web build
```

Write your code (see [examples](examples) and [documentation](https://docs.rs/bugsalot/) for more code):
```rust
use bugsalot::*;

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


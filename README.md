# datatest-stable

[![datatest-stable on crates.io](https://img.shields.io/crates/v/datatest-stable)](https://crates.io/crates/datatest-stable)
[![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/datatest-stable/)
[![Documentation (main)](https://img.shields.io/badge/docs-main-purple)](https://nextest-rs.github.io/datatest-stable/rustdoc/datatest_stable/)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](../LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](../LICENSE-MIT)

`datatest-stable` is a very simple test harness intended to write data-driven tests, where
individual test cases are specified as files and not as code. Given:
* a test `my_test` that accepts a path as input
* a directory to look for files in
* a pattern to match files on

`datatest-stable` will call the `my_test` function once per matching file in the directory.

`datatest-stable` works with [cargo nextest](https://nexte.st/), and is part of the [nextest-rs
organization](https://github.com/nextest-rs/) on GitHub.

## Usage

1. Configure the test target by setting `harness = false` in `Cargo.toml`:

```toml
[[test]]
name = "<test target name>"
harness = false
```

2. Call the `datatest_stable::harness!(testfn, root, pattern)` macro with the following
parameters:
* `testfn` - The test function to be executed on each matching input. This function must have
  the type `fn(&Path) -> datatest_stable::Result<()>`
* `root` - The path to the root directory where the input files (fixtures) live. This path is
  relative to the root of the crate.
* `pattern` - the regex used to match against and select each file to be tested.

The three parameters can be repeated if you have multiple sets of data-driven tests to be run:
`datatest_stable::harness!(testfn1, root1, pattern1, testfn2, root2, pattern2)`

## Examples

This is an example test. Use it with `harness = false`.

```rust
use std::path::Path;

fn my_test(path: &Path) -> datatest_stable::Result<()> {
    // ... write test here

    Ok(())
}

datatest_stable::harness!(my_test, "path/to/fixtures", r"^.*/*");
```

## Minimum supported Rust version (MSRV)

The minimum supported Rust version is **Rust 1.58**. MSRV bumps may be accompanied by a minor
version update; at any time, at least the last 3 stable versions of Rust will be supported.

## See also

* [`datatest`](https://crates.io/crates/datatest): the original inspiration for this crate,
  with a better UI and more features but targeting nightly Rust
* [Data-driven testing](https://en.wikipedia.org/wiki/Data-driven_testing)

## License

This project is available under the terms of either the [Apache 2.0 license](LICENSE-APACHE) or the [MIT
license](LICENSE-MIT).

<!--
README.md is generated from README.tpl by cargo readme. To regenerate:

cargo install cargo-readme
cargo readme > README.md
-->

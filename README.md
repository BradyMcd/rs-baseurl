# BaseUrl

A library for the programming language [Rust](https://www.rust-lang.org).

This is a thin wrapper over the Url crate which eliminates the potential for errors related to valid
Url which cannot be a base by checking that the Url can be a base on creation. As such, a BaseUrl never
fails when doing things like calling set_path()

## Usage

In any Rust project managed by Cargo add the following to your Cargo.toml ```[dependencies]``` section:
```
base_url="0.0.5"
```
The package exposes a single package, base_url
```rust
extern crate base_url
```
BaseUrl can be created by converting from a String or a Url, these conversions are implemented using
the traits found in the [try_from](https://crates.io/crates/try_from) crate.

## What's Missing?

This is at version 0.0.5 for a reason, there are things to be added before I'm comfortable claiming 
this library is at a proper first version.
Documentation for one, is missing. Mostly that will just be copied from the main Url crate with some
changes where fewer (or no) errors are possible
Tests should also be added, even though this is mostly just a thin wrapper over the Url type proving
that CannotBeBase errors are truly gone when using the library is something which should be done
Finally there are some functions which are still missing, they're noted in a TODO in the code


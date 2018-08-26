# BaseUrl

A library for the programming language [Rust](https://www.rust-lang.org).

This is a thin wrapper over the [Url](https://crates.io/crates/url) crate which eliminates the potential
for errors related to valid Url which cannot be a base by checking that the Url can be a base on 
creation. As such, a BaseUrl never fails when doing things like calling ```set_path()```

## Usage

In any Rust project managed by Cargo add the following to your Cargo.toml ```[dependencies]``` section:
```
base_url="0.0.7"
```
The package exposes base_url which defines the BaseUrl structure at it's root and also re-exports the 
rust-url crate
```rust
extern crate base_url;
use base_url::BaseUrl;
use base_url::Url;
```
BaseUrl can be created by converting from a String or a Url, these conversions are implemented using
the traits found in the [try_from](https://crates.io/crates/try_from) crate.

## What's Missing?

This is at version 0.0.7 for a reason, there are things to be added before I'm comfortable claiming 
this library is at a proper first version.  
Tests should also added, even though this is mostly just a thin wrapper over the Url type proving
that CannotBeBase errors are truly gone when using the library is something which should be done  
Finally there are some functions which are still missing, anything which requires me to reimplement the ```parse()```
function isn't being touched until I really need to, notably ```join()``` isn't available yet.  
Some of the nominclature also needs to be changed to better reflect what the two Error types are in
reference to, specifically the UrlError type is going to change since we're re-exporting Url and it
has nothing to do with that crate.


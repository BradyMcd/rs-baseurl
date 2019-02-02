# BaseUrl

A library for the programming language [Rust](https://www.rust-lang.org).

This is a thin wrapper over the [Url](https://crates.io/crates/url) crate which eliminates the potential
for errors related to valid Url which cannot be a base by checking that the Url can be a base on 
creation. As such, a BaseUrl never fails when doing things like calling ```set_path()```

## Usage

In any Rust project managed by Cargo add the following to your Cargo.toml ```[dependencies]``` section:
```
base_url="1.0.0"
```

If you're building in nightly Rust it's suggested you instead use the nightly branch, this will avoid
an uneeded dependency in ```try_from``` as well as possible name collisions between it and the std
implementation.
```
base_url={ git='https://github.com/bradymcd/rs-baseurl', branch = 'nightly' }
```

The package exposes base_url which defines the BaseUrl structure at it's root and also re-exports the 
content of the rust-url and try_from crates by default.
```rust
extern crate base_url;
use base_url::BaseUrl;
use base_url::Url;
use base_url::TryFrom;
```
There are also two related error types, one a re-export of the rust-url error type, the other expresses
errors related to host suitability when converting into a BaseUrl.
```rust
use base_url::ParseError;
use base_url::BaseUrlError;
```

BaseUrl can be created by converting from a String or a Url using TryFrom. In cases where the source is
known to be well formed an implementation of From is also supplied, but know that these implementations
can panic.

Once you have a BaseUrl you can do basically anything with it that you could with a normal Url so long
as what you want to do wouldn't cause it to lose it's ability to be a base.

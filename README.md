# BaseUrl

base_url is a thin wrapper around [rust-url](https://github.com/servo/rust-url), which itself
implements the [URL Standard](https://url.spec.whatwg.org/). The goal of base_url is to implement
a strict subset of that standard to remove redundant error checks related to the base-suitability of a
given URL.

## What is a BaseUrl

A BaseUrl can be converted from any Url which has a host following its scheme (and possibly 
credentials) in roughly the form foo.bar. In practical terms, if you found it on the Internet it will 
probably work and if you're referring to a resource 'somewhere else' it will probably work, mailto:
being a notable exception. Internally any Url which returns false on a ```.cannot_be_a_base()``` call 
will convert.

## Acquiring a BaseUrl object

A BaseUrl object may be acquired by either converting a Url or &str using TryFrom, in Stable this is
accomplished by depending on [try_from](https://crates.io/crates/try_from). If a &str cannot be parsed
into a Url object a BaseUrlError::ParseError will be returned which wraps the underlying ParseError 
type implemented by rust-url.

```
use base_url::{ BaseUrl, BaseUrlError, Url, ParseError, TryFrom };

assert!( BaseUrl::try_from( "http://[:::1]" ) == Err( BaseUrlError::ParseError( ParseError::InvalidIpv6Address ) ) );
```

That's a bit unwieldly, so it's suggested that you prefer first parsing the &str into a Url and
converting that object into a BaseUrl, allowing you to deal with errors related to parsing separately
from errors related to base suitability.

```
use base_url::{ BaseUrl, BaseUrlError, Url, TryFrom };

# fn run( ) -> Result< (), BaseUrlError > {
let url:Url = Url::parse( "data:text/plain,Hello?World#" )?;

assert!( BaseUrl::try_from( url ) == Err( BaseUrlError::CannotBeBase ) );
# Ok( () )
# }
# run( );
```

Once we have a BaseUrl we can do (almost) anything we could with a normal Url and with fewer functions
admitting potential failures and fewer calls to ```.unwrap()```.

If you do need to use some call which can remove the host or otherwise cause the Url to lose base 
suitability the wrapped Url can be acquired using ```.from()```.

## Usage

In any Rust project managed by Cargo add the following to your Cargo.toml ```[dependencies]``` section:
```
base_url="^1.0.0"
```

If you're building in nightly Rust it's suggested you instead use the nightly branch, this will avoid
an uneeded dependency in the try_from crate as well as possible name collisions between it and the 
standard implementation.
```
base_url={ git='https://github.com/bradymcd/rs-baseurl', branch = 'nightly' }
```

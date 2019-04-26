# BaseUrl

base_url is a thin wrapper around [rust-url](https://github.com/servo/rust-url), which itself
implements the [URL Standard](https://url.spec.whatwg.org/). The goal of base_url is to implement
a strict subset of that standard to remove redundant error checks related to the base-suitability of a
given URL.

## What is a BaseUrl

A BaseUrl can be converted from any Url which can refer to a remote resource. In practical terms, if 
you found it on the Internet it will probably work and if you're referring to a resource 'somewhere 
else' it will probably work, mailto: being a notable exception. 

Internally any Url which returns false on a ```.cannot_be_a_base()``` call and true on a 
```.has_authority()``` call will convert. That means setting things like credentials and port numbers
cannot fail and likewise setting the path relative to the host cannot fail.

## Acquiring a BaseUrl object

A BaseUrl object may be acquired by either converting a Url or &str using TryFrom. If a &str cannot be 
parsed into a Url object a BaseUrlError::ParseError will be returned which wraps the underlying 
ParseError type implemented by rust-url.

```
use base_url::{ BaseUrl, BaseUrlError, Url, ParseError, TryFrom };

assert!( BaseUrl::try_from( "http://[:::1]" ) == Err( BaseUrlError::ParseError( ParseError::InvalidIpv6Address ) ) );
```

That's a bit unwieldly, so it's suggested that you prefer first parsing the &str into a Url and
converting that object into a BaseUrl, allowing you to deal with errors related to parsing separately
from errors related to base suitability.

```
use base_url::{ BaseUrl, BaseUrlError, Url, TryFrom };

let url:Url = Url::parse( "data:text/plain,Hello?World#" )?;
assert!( BaseUrl::try_from( url ) == Err( BaseUrlError::CannotBeBase ) );

let url:Url = Url::parse( "https://example.org/" )?;
let baseurl = BaseUrl::from( url )?;
assert!( baseurl.as_str( ), "https://example.org/ " );
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

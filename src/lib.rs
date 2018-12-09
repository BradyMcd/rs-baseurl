/*!

base_url is a thin wrapper around [rust-url](https://github.com/servo/rust-url), which itself
implements the [URL Standard](https://url.spec.whatwg.org/). The goal of base_url is to implement
a strict subset of that standard to remove redundant error checks related to the base-suitability of a
given URL.


# Acquiring a BaseUrl object

A BaseUrl object may be acquired by either converting a Url or &str using the TryInto/TryFrom traits.
If a &str cannot be parsed into a Url object a BaseUrlError::ParseError will be returned which wraps the
underlying ParseError type implemented by rust-url.

```
use base_url::{ BaseUrl, BaseUrlError, Url, ParseError };

assert!( BaseUrl::try_from( "http://[:::1]" ) == Err( BaseUrlError::ParseError( ParseError::InvalidIpv6Address ) ) );
```

That's a bit unwieldly, so it's suggested that you prefer first parsing the &str into a Url and
converting that object into a BaseUrl, allowing you to deal with errors related to parsing separately
from errors related to base suitability.

```
use base_url::{ BaseUrl, BaseUrlError, Url };

# fn run( ) -> Result< (), BaseUrlError > {
let url:Url = Url::parse( "data:text/plain,Hello?World#" )?;

assert!( BaseUrl::try_from( url ) == BaseUrlError::CannotBeBase );
# Ok( () )
# }
# run( );
```

Once we have a BaseUrl we can do (almost) anything we could with a normal Url and with fewer functions
admitting potential failures


*/

pub extern crate url;
pub extern crate try_from;

#[cfg( feature = "robot_conversion" )]
pub mod robotparser;

#[cfg( feature = "sitemap_conversion" )]
pub mod sitemap;

pub use url::{ Url, ParseError };

use url::{ UrlQuery, PathSegmentsMut };
use url::form_urlencoded::{Parse, Serializer};
use try_from::TryFrom;

pub use url::{ Host };

use std::str::Split;
use std::net::IpAddr;
use std::fmt::{Formatter, Display, Result as FormatResult};

/// A representation of the origin of a BaseUrl
pub type OriginTuple = ( String, Host<String>, u16 );

#[derive(Debug)]
pub enum BaseUrlError {
    /// If the Url supplied cannot be a base this error is returned
    CannotBeBase,
    /// If a supplied &str cannot be parsed by the parser in the main Url crate this error is returned
    ParseError( ParseError ),
}

/// Any Url which has a host and so can be supplied as a base url
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BaseUrl {
    url:Url,
}

impl From<BaseUrl> for Url {
    fn from( url:BaseUrl ) -> Url {
        url.url
    }
}

impl TryFrom<Url> for BaseUrl {
    type Err = BaseUrlError;
    fn try_from( url:Url ) -> Result< BaseUrl, Self::Err > {
        if url.cannot_be_a_base( ) {
            Err( BaseUrlError::CannotBeBase )
        } else {
            Ok( BaseUrl{ url:url } )
        }
    }
}

impl<'a> TryFrom<&'a str> for BaseUrl {
    type Err = BaseUrlError;

    fn try_from( url:&'a str ) -> Result< BaseUrl, Self::Err > {
        match Url::parse( url ) {
            Ok( u ) => BaseUrl::try_from( u ),
            Err( e ) => Err( BaseUrlError::ParseError( e ) ),
        }
    }
}

impl BaseUrl {

    /// Return the serialization of this BaseUrl
    ///
    /// This is fast, since internally the Url stores the serialization already
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, Url };
    ///# use base_url::{ BaseUrlError };
    ///# fn run( ) -> Result< (), BaseUrlError > {
    /// let url_str = "https://example.org/"
    /// let host = BaseUrl::try_from( url_str )?;
    /// assert_eq!( host.as_str( ), url_str );
    ///# Ok( () )
    ///# }
    ///# run( );
    /// ```
    pub fn as_str( &self ) -> &str {
        self.url.as_str( )
    }

    /// Return the serialization of this BaseUrl
    ///
    /// This consumes the BaseUrl and takes ownership of the String
    ///
    /// # Examples
    /// ```rust
    /// use base_url::BaseUrl;
    ///# use base_url::{ ParseError };
    ///# fn run( ) -> Result< (), BaseUrlError > {
    /// let url_str = "https://example.org/"
    /// let host = BaseUrl::try_from( url_str )?;
    /// assert_eq!( host.into_string, url_str );
    ///# Ok( () )
    ///# }
    ///# run( );
    /// ```
    pub fn into_string( self ) -> String {
        self.url.into_string( )
    }


    /// Returns the BaseUrl's scheme, host and port as a tuple
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, OriginTuple, Host };
    ///# use base_url::BaseUrlError;
    ///# fn run( ) -> Result< (), BaseUrlError > {
    /// let url = BaseUrl::try_from( "ftp://example.org/foo" );
    ///
    /// assert_eq!( url.origin( ),
    ///             ( "ftp".into( ),
    ///               Host::Domain( "example.org".into( ) ),
    ///               21 ) );
    ///# Ok( () )
    ///# }
    ///# run( );
    /// ```
    pub fn origin( &self ) -> OriginTuple {
        match self.url.origin( ) {
            url::Origin::Opaque( _ ) => { panic!( "Some sorcery occurred, please raise an issue at https://github.com/bradymcd/rs-baseurl" ) }
            url::Origin::Tuple( scheme, host, port ) => {
                ( scheme, host, port )
            }
        }
    }


    /// Returns the scheme of the given BaseUrl, lower-cased, as an ASCII string without the ':'
    /// delimiter
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::BaseUrl;
    ///# use base_url::BaseUrlError;
    ///# fn run( ) -> Result< (), BaseUrlError > {
    /// let url = BaseUrl::try_from( "https://example.org" )?;
    /// assert_eq!( url.scheme, "https".into( ) );
    ///# Ok( () )
    ///# }
    ///# run( );
    /// ```
    pub fn scheme( &self ) -> &str {
        self.url.scheme( )
    }

    //TODO: Examples below this point

    /// Set the BaseUrl's scheme
    ///
    /// Does nothing and returns Err() if the specified scheme does not match the regular expression
    /// [a-zA-Z][a-zA-Z0-9+.-]+
    ///
    ///
    pub fn set_scheme( &mut self, scheme: &str ) -> Result< (), () > {
        self.url.set_scheme( scheme )
    }

    /// Return the username for this BaseUrl. If no username is set an empty string is returned
    ///
    ///
    pub fn username( &self ) -> &str {
        self.url.username( )
    }

    /// Change the username of this BaseUrl.
    ///
    ///
    pub fn set_username( &mut self, username:&str ) {
        self.url.set_username( username ).expect( "The impossible happened" );
    }

    /// Optionally returns the password associated with this BaseUrl as a percent-encoded ASCII string.
    ///
    ///
    pub fn password( &self ) -> Option< &str > {
        self.url.password( )
    }

    /// Change the password of this BaseUrl. Use None to remove the password field.
    ///
    ///
    pub fn set_password( &mut self, password:Option< &str > ) {
        self.url.set_password( password ).expect( "The impossible happened" );
    }

    /// Returns the domain or IP address for this BaseUrl as a string.
    ///
    /// See also the host() method
    ///
    ///
    pub fn host_str( &self ) -> &str {
        self.url.host_str( ).unwrap( )
    }

    /// Returns the host for this BaseUrl in an enumerated type.
    ///
    ///
    pub fn host( &self ) -> Host< &str > {
        self.url.host( ).unwrap( )
    }

    /// Changes the host for this BaseUrl. If there is any error parsing the provided string no action
    /// is taken and Err() is returned
    ///
    ///
    pub fn set_host( &mut self, host:&str ) -> Result< (), () > {
        match self.url.set_host( Some( host ) ) {
            Ok( _ ) => Ok( () ),
            Err( _ ) => Err( () ),
        }
    }

    /// Change this BaseUrl's host to the given Ip address.
    ///
    /// This skips the parsing step compared to calling set_host()
    ///
    ///
    pub fn set_ip_host( &mut self, address:IpAddr ) {
        self.url.set_ip_host( address ).expect( "The impossible occurred" );
    }

    /// Return's the domain string of this BaseUrl. Returns None if the host is an Ip address rather
    /// than a domain name.
    ///
    pub fn domain( &self ) -> Option< &str > {
        self.url.domain( )
    }

    /// Optionally return's the port number of this BaseUrl.
    ///
    pub fn port( &self ) -> Option< u16 > {
        self.url.port( )
    }

    /// Return's the port number of this BaseUrl. If no port number is present a guess is made based
    /// on the scheme, if no guess can be made None is returned.
    ///
    pub fn port_or_known_default( &self ) -> Option< u16 > {
        self.url.port_or_known_default( )
    }

    /// Change this BaseUrl's port.
    ///
    pub fn set_port( &mut self, port:Option< u16 > ) {
        self.url.set_port( port ).expect( "The impossible happened" )
    }

    /// Return's the path of this BaseUrl, percent-encoded. Path strings will start with '/' and
    /// continue with '/' separated path segments.
    ///
    pub fn path( &self ) -> &str {
        self.url.path( )
    }

    /// Return's an iterator through each of this BaseUrl's path segments. Path segments do not contain
    /// the separating '/' characters and may be empty, often on the last entry.
    ///
    pub fn path_segments( &self ) -> Split<char> {
        self.url.path_segments( ).unwrap( )
    }

    /// Change this BaseUrl's path
    ///
    ///
    pub fn set_path( &mut self, path:&str ) {
        self.url.set_path( path )
    }


    /// Returns an object with methods to manipulate this BaseUrl's path segments.
    ///
    ///
    pub fn path_segments_mut( &mut self ) -> PathSegmentsMut {
        self.url.path_segments_mut( ).unwrap( )
    }

    /// Optionally return's this BaseUrl's percent-encoded query string.
    ///
    ///
    pub fn query( &self ) -> Option< &str > {
        self.url.query( )
    }

    /// Parse the BaseUrl's query string and return an iterator over all found (key, value) pairs.
    ///
    ///
    pub fn query_pairs( &self ) -> Parse {
        self.url.query_pairs( )
    }

    /// Change this BaseUrl's query string.
    ///
    ///
    pub fn set_query( &mut self, query:Option<&str> ) {
        self.url.set_query( query )
    }

    /// Returns an object with a method chaining API. These methods manipulate the query string of the
    /// BaseUrl as a sequence of (key, value) pairs.
    ///
    ///
    pub fn query_pairs_mut( &mut self ) -> Serializer< UrlQuery > {
        self.url.query_pairs_mut( )
    }

    /// Optionally returns this BaseUrl's fragment identifier.
    ///
    ///
    pub fn fragment( &self ) -> Option< &str > {
        self.url.fragment( )
    }

    /// Change this BaseUrl's fragment identifier.
    ///
    /// The fragment is any text placed after a `#` symbol in the Url. It is meant to refer to a
    /// secondary resource.
    ///
    /// This is often not sent to the server where it is used in http: and similar schemes.
    ///
    pub fn set_fragment( &mut self, fragment:Option<&str> ) {
        self.url.set_fragment( fragment )
    }

    /* TODO: possibly
    pub fn with_default_port <F> ( &self, f:F ) -> Result<HostAndPort<&str>>
     */



}

impl Display for BaseUrl {
    fn fmt( &self, formatter: &mut Formatter ) -> FormatResult {
        self.url.fmt( formatter )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

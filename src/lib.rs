//:393
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
admitting potential failures


*/

pub extern crate url;

#[cfg( all( feature = "_try_from", not( feature = "nightly" ) ) )]
pub extern crate try_from;
#[cfg( all( feature = "_try_from", not( feature = "nightly" ) ) )]
pub use try_from::TryFrom;

#[cfg( feature = "robot_conversion" )]
pub mod robotparser;

#[cfg( feature = "sitemap_conversion" )]
pub mod sitemap;

pub use url::{ Url, ParseError };

use url::{ UrlQuery, PathSegmentsMut };
use url::form_urlencoded::{Parse, Serializer};
pub use url::{ Host };

use std::str::Split;
use std::net::IpAddr;
use std::fmt::{Formatter, Display, Result as FormatResult};

/// A representation of the origin of a BaseUrl
pub type OriginTuple = ( String, Host<String>, u16 );

#[derive(Debug, PartialEq)]
pub enum BaseUrlError {
    /// If the Url supplied cannot be a base this error is returned
    CannotBeBase,
    /// If a supplied &str cannot be parsed by the parser in the main Url crate this error is returned
    ParseError( ParseError ),
}

/// Any Url which has a host and so can be supplied as a base url
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BaseUrl {
    url: Url,
}

impl From<BaseUrl> for Url {
    fn from( url: BaseUrl ) -> Self {
        url.url
    }
}

#[cfg( any( feature = "_try_from", feature = "nightly" ) )]
impl TryFrom<Url> for BaseUrl {
    type Err = BaseUrlError;
    fn try_from( url: Url ) -> Result< Self, Self::Err > {
        if url.cannot_be_a_base( ) {
            Err( BaseUrlError::CannotBeBase )
        } else {
            Ok( BaseUrl{ url: url } )
        }
    }
}

#[cfg( any( feature = "_try_from", feature = "nightly" ) )]
impl<'a> TryFrom<&'a str> for BaseUrl {
    type Err = BaseUrlError;

    fn try_from( url: &'a str ) -> Result< Self, Self::Err > {
        match Url::parse( url ) {
            Ok( u ) => BaseUrl::try_from( u ),
            Err( e ) => Err( BaseUrlError::ParseError( e ) ),
        }
    }
}

/// This is a fallible conversion and CAN panic
impl From<Url> for BaseUrl {
    fn from( url: Url ) -> Self {
        if url.cannot_be_a_base( ) {
            panic!()
        }else{
            BaseUrl{ url: url }
        }
    }
}

/// This is a fallible conversion and CAN panic
impl<'a> From<&'a str> for BaseUrl {
    fn from( url: &'a str ) -> Self {
        match Url::parse( url ) {
            Ok( u ) => BaseUrl::from( u ),
            Err( _e ) => panic!(),
        }
    }
}

impl From< ParseError > for BaseUrlError {
    fn from( err:ParseError ) -> Self {
        BaseUrlError::ParseError( err )
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
    /// use base_url::{ BaseUrl, BaseUrlError, Url, TryFrom };
    ///# fn run( ) -> Result< (), BaseUrlError > {
    /// let url_str = "https://example.org/";
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
    /// use base_url::{ BaseUrl, BaseUrlError, Url, ParseError, TryFrom };
    ///# fn run( ) -> Result< (), BaseUrlError > {
    /// let url_str = "https://example.org/";
    /// let host = BaseUrl::try_from( url_str )?;
    /// assert_eq!( host.into_string( ), url_str );
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
    /// use base_url::{ BaseUrl, OriginTuple, Host, TryFrom };
    ///# use base_url::BaseUrlError;
    ///# fn run( ) -> Result< (), BaseUrlError > {
    /// let url = BaseUrl::try_from( "ftp://example.org/foo" )?;
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
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< (), BaseUrlError > {
    /// let url = BaseUrl::try_from( "https://example.org" )?;
    /// assert_eq!( url.scheme( ), "https" );
    ///# Ok( () )
    ///# }
    ///# run( );
    /// ```
    pub fn scheme( &self ) -> &str {
        self.url.scheme( )
    }

    /// Strip out any present username, password, query and fragment information from this BaseUrl
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< (), BaseUrlError > {
    /// let mut url = BaseUrl::try_from( "http://brady:hunter3@example.org/foo?query=1#fragment=2" )?;
    /// url.strip( );
    /// assert_eq!( url.as_str( ), "http://example.org/foo" );
    ///# Ok( () )
    ///# }
    ///# run( );
    /// ```
    pub fn strip( &mut self ) {
        self.set_username( "" );
        self.set_password( None );
        self.set_query( None );
        self.set_fragment( None );
    }

    /// Strips a BaseUrl down to only the host and scheme.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< (), BaseUrlError > {
    /// let mut url = BaseUrl::try_from( "http://brady:hunter3@example.org:8080/foo?query=1#fragment=2" )?;
    /// url.make_host_only( );
    /// assert_eq!( url.as_str( ), "http://example.org/" );
    ///# Ok( () )
    ///# }
    ///# run( );
    /// ```
    pub fn make_host_only( &mut self ) {
        self.strip( );
        self.set_path( "" );
        self.set_port( None );
    }


    /// Set the BaseUrl's scheme
    ///
    /// Does nothing and returns Err() if the specified scheme does not match the regular expression
    /// [a-zA-Z][a-zA-Z0-9+.-]+
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let mut url = BaseUrl::try_from( "http://example.org/" )?;
    /// url.set_scheme( "https" );
    /// assert_eq!( url.as_str( ), "https://example.org/" );
    ///# Ok( () )
    ///# }
    ///# run( );
    /// ```
    pub fn set_scheme( &mut self, scheme: &str ) -> Result< (), () > {
        self.url.set_scheme( scheme )
    }

    /// Return the username for this BaseUrl. If no username is set an empty string is returned
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "https://brady@example.org/foo" )?;
    /// assert_eq!( url.username( ), "brady" );
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```
    pub fn username( &self ) -> &str {
        self.url.username( )
    }

    /// Change the username of this BaseUrl.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let mut url = BaseUrl::try_from( "http://example.org/" )?;
    /// url.set_username( "brady" );
    /// assert_eq!( url.as_str( ), "http://brady@example.org/" );
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```
    pub fn set_username( &mut self, username:&str ) {
        self.url.set_username( username ).expect( "The impossible happened" );
    }

    /// Optionally returns the password associated with this BaseUrl as a percent-encoded ASCII string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ Baseurl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    ///# Ok( () )
    /// let url = BaseUrl::try_from( "https://brady:hunter3@example.org/" )?;
    /// assert_eq!( url.password( ), Some( "hunter3" ) );
    ///
    ///# }
    ///# run( );
    /// ```
    ///
    /// ```rust
    /// use base_url{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "https://example.org/" )?;
    /// assert( url.password( ).is_none( ) );
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```
    pub fn password( &self ) -> Option< &str > {
        self.url.password( )
    }

    /// Change the password of this BaseUrl. Use None to remove the password field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let mut url = BaseUrl::try_from( "http://brady@example.org/" )?;
    /// url.set_password( Some( "hunter3" ) );
    /// assert_eq!( url.as_str( ), "http://brady:hunter3@example.org/" );
    /// url.set_password( None );
    /// assert_eq!( url.password( ), None );
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```
    pub fn set_password( &mut self, password:Option< &str > ) {
        self.url.set_password( password ).expect( "The impossible happened" );
    }

    /// Returns the domain or IP address for this BaseUrl as a string.
    ///
    /// See also the host() method
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "http://brady@example.org/foo" )?;
    /// assert_eq!( url.host_str( ), "example.org" );
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```
    pub fn host_str( &self ) -> &str {
        self.url.host_str( ).unwrap( )
    }
    //NOTE NOTE NOTE: STUBS AHEAD
    /// Returns the host for this BaseUrl in an enumerated type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```


    pub fn host( &self ) -> Host< &str > {
        self.url.host( ).unwrap( )
    }

    /// Changes the host for this BaseUrl. If there is any error parsing the provided string no action
    /// is taken and Err() is returned
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```


    pub fn set_host( &mut self, host:&str ) -> Result< (), () > {
        match self.url.set_host( Some( host ) ) {
            Ok( _ ) => Ok( () ),
            Err( _ ) => Err( () ),
        }
    }

    /// Change this BaseUrl's host to the given Ip address.
    ///
    /// This skips the parsing step compared to calling set_host()
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```

    
    pub fn set_ip_host( &mut self, address:IpAddr ) {
        self.url.set_ip_host( address ).expect( "The impossible occurred" );
    }

    /// Return's the domain string of this BaseUrl. Returns None if the host is an Ip address rather
    /// than a domain name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```


    pub fn domain( &self ) -> Option< &str > {
        self.url.domain( )
    }

    /// Optionally return's the port number of this BaseUrl.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```


    pub fn port( &self ) -> Option< u16 > {
        self.url.port( )
    }

    /// Return's the port number of this BaseUrl. If no port number is present a guess is made based
    /// on the scheme, if no guess can be made None is returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```


    pub fn port_or_known_default( &self ) -> Option< u16 > {
        self.url.port_or_known_default( )
    }

    /// Change this BaseUrl's port.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```


    pub fn set_port( &mut self, port:Option< u16 > ) {
        self.url.set_port( port ).expect( "The impossible happened" )
    }

    /// Return's the path of this BaseUrl, percent-encoded. Path strings will start with '/' and
    /// continue with '/' separated path segments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```


    pub fn path( &self ) -> &str {
        self.url.path( )
    }

    /// Return's an iterator through each of this BaseUrl's path segments. Path segments do not contain
    /// the separating '/' characters and may be empty, often on the last entry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```


    pub fn path_segments( &self ) -> Split<char> {
        self.url.path_segments( ).unwrap( )
    }

    /// Change this BaseUrl's path
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```


    pub fn set_path( &mut self, path:&str ) {
        self.url.set_path( path )
    }


    /// Returns an object with methods to manipulate this BaseUrl's path segments.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```


    pub fn path_segments_mut( &mut self ) -> PathSegmentsMut {
        self.url.path_segments_mut( ).unwrap( )
    }

    /// Optionally return's this BaseUrl's percent-encoded query string.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```


    pub fn query( &self ) -> Option< &str > {
        self.url.query( )
    }

    /// Parse the BaseUrl's query string and return an iterator over all found (key, value) pairs.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```


    pub fn query_pairs( &self ) -> Parse {
        self.url.query_pairs( )
    }

    /// Change this BaseUrl's query string.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```


    pub fn set_query( &mut self, query:Option<&str> ) {
        self.url.set_query( query )
    }

    /// Returns an object with a method chaining API. These methods manipulate the query string of the
    /// BaseUrl as a sequence of (key, value) pairs.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```


    pub fn query_pairs_mut( &mut self ) -> Serializer< UrlQuery > {
        self.url.query_pairs_mut( )
    }

    /// Optionally returns this BaseUrl's fragment identifier.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```
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
    /// # Examples
    ///
    /// ```rust
    /// use base_url::{ BaseUrl, BaseUrlError, TryFrom };
    ///# fn run( ) -> Result< ( ), BaseUrlError > {
    /// let url = BaseUrl::try_from( "" )?;
    ///# Ok( () );
    ///# }
    ///# run( );
    /// ```
    pub fn set_fragment( &mut self, fragment:Option<&str> ) {
        self.url.set_fragment( fragment )
    }

}

impl Display for BaseUrl {
    fn fmt( &self, formatter: &mut Formatter ) -> FormatResult {
        self.url.fmt( formatter )
    }
}

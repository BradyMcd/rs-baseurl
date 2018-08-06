/// BaseUrl are any valid Url which can be a base, this eliminates a lot of possible errors and
/// redundant error checks when doing things like setting a path and allows certain related fail-free
/// methods.
/// This does have the effect of restricting how you can interact with a BaseUrl compared to other Url
/// since certain methods make no sense when you are definitely dealing with a base-capable url
use url::{ Url, ParseError, UrlQuery, PathSegmentsMut };
use url::form_urlencoded::{Parse, Serializer};
use try_from::TryFrom;

pub use url::{Host, Origin};

use std::str::Split;
use std::convert::Into;
use std::net::IpAddr;
use std::fmt::{Formatter, Display, Result as FormatResult};

pub enum UrlError {
    /// If the Url supplied cannot be a base this error is returned
    CannotBeBase,
    ///
    ParseError( ParseError ),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BaseUrl {
    url:Url,
}

/* TODO:Missing traits
 * AsRef
 */

impl Into<Url> for BaseUrl {
    fn into( self ) -> Url {
        self.url
    }
}

impl TryFrom<Url> for BaseUrl {
    type Err = UrlError;
    fn try_from( url:Url ) -> Result< BaseUrl, Self::Err > {
        if url.cannot_be_a_base( ) {
            Err( UrlError::CannotBeBase )
        } else {
            Ok( BaseUrl{ url:url } )
        }
    }
}

impl<'a> TryFrom<&'a str> for BaseUrl {
    type Err = UrlError;

    fn try_from( url:&'a str ) -> Result< BaseUrl, Self::Err > {
        match Url::parse( url ) {
            Ok( u ) => BaseUrl::try_from( u ),
            Err( e ) => Err( UrlError::ParseError( e ) ),
        }
    }
}

impl BaseUrl {

    pub fn as_str( &self ) -> &str {
        self.url.as_str( )
    }

    pub fn into_string( self ) -> String {
        self.url.into_string( )
    }

    pub fn origin( &self ) -> Origin {
        self.url.origin( )
    }

    pub fn scheme( &self ) -> &str {
        self.url.scheme( )
    }

    pub fn set_scheme( &mut self, scheme: &str ) -> Result< (), () > {
        self.url.set_scheme( scheme )
    }

    pub fn has_authority( &self ) -> bool {
        self.url.has_authority( )
    }

    pub fn username( &self ) -> &str {
        self.url.username( )
    }

    pub fn set_username( &mut self, username:&str ) {
        self.url.set_username( username ).expect( "The impossible happened" );
    }

    pub fn password( &self ) -> Option< &str > {
        self.url.password( )
    }

    pub fn set_password( &mut self, password:Option< &str > ) {
        self.url.set_password( password ).expect( "The impossible happened" );
    }

    pub fn host_str( &self ) -> &str {
        self.url.host_str( ).unwrap( )
    }

    pub fn host( &self ) -> Host< &str > {
        self.url.host( ).unwrap( )
    }

    pub fn set_host( &mut self, host:Option<&str> ) -> Result< (), ParseError > {
        //TODO: Change to a more informative error type which doesn't admit any CannotBeBase potential
        self.url.set_host( host )
    }

    pub fn set_ip_host( &mut self, address:IpAddr ) {
        self.url.set_ip_host( address ).expect( "The impossible occurred" );
    }

    pub fn domain( &self ) -> Option< &str > {
        self.url.domain( )
    }

    pub fn port( &self ) -> Option< u16 > {
        self.url.port( )
    }

    pub fn port_or_known_default( &self ) -> Option< u16 > {
        self.url.port_or_known_default( )
    }

    pub fn set_port( &mut self, port:Option< u16 > ) -> Result< (),() > {
        self.url.set_port( port )
    }

    pub fn path( &self ) -> &str {
        self.url.path( )
    }

    pub fn path_segments( &self ) -> Split<char> {
        self.url.path_segments( ).unwrap( )
    }

    pub fn set_path( &mut self, path:&str ) {
        self.url.set_path( path )
    }

    pub fn path_segments_mut( &mut self ) -> PathSegmentsMut {
        self.url.path_segments_mut( ).unwrap( )
    }

    pub fn query( &self ) -> Option< &str > {
        self.url.query( )
    }

    pub fn query_pairs( &self ) -> Parse {
        self.url.query_pairs( )
    }

    pub fn set_query( &mut self, query:Option<&str> ) {
        self.url.set_query( query )
    }

    pub fn query_pairs_mut( &mut self ) -> Serializer< UrlQuery > {
        self.url.query_pairs_mut( )
    }

    pub fn fragment( &self ) -> Option< &str > {
        self.url.fragment( )
    }

    pub fn set_fragment( &mut self, fragment:Option<&str> ) {
        self.url.set_fragment( fragment )
    }

    /* TODO:
    pub fn with_default_port <F> ( &self, f:F ) -> Result<HostAndPort<&str>>

    pub fn from_file_path<P: AsRef<Path>>( path:P ) -> Result< Url, () >

    pub fn from_directory_path<P: AsRef<Path>>( path:P ) -> Result< Url, () >

    pub fn to_file_path( &self ) -> Result< PathBuf, () >
     */



}

impl Display for BaseUrl {
    fn fmt( &self, formatter: &mut Formatter ) -> FormatResult {
        self.url.fmt( formatter )
    }
}


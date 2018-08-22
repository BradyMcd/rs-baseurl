
extern crate url;
extern crate try_from;

pub use url::{ Url, ParseError };

pub mod base_url;

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

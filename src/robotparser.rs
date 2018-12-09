
use BaseUrl;
use url::Url;

extern crate robotparser;
use self::robotparser::RobotFileParser;

impl<'a> From< BaseUrl > for RobotFileParser<'a> {
    /// Conversion into a RobotFileParser from the BaseUrl type
    fn from( url:BaseUrl ) -> RobotFileParser<'a> {
        RobotFileParser::<'a>::new( Url::from( url ) )
    }
}


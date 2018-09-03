
use BaseUrl;
use url::Url;

#[cfg( feature = "robot_conversion" )]
extern crate robotparser;
use self::robotparser::RobotFileParser;

#[cfg( feature = "sitemap_conversion" )]
extern crate sitemap;
use self::sitemap::structs::{Location, SiteMapEntry, UrlEntry, LastMod, ChangeFreq, Priority};

#[cfg( feature = "robot_conversion" )]
impl<'a> From< BaseUrl > for RobotFileParser<'a> {
    /// Conversion into a RobotFileParser from the BaseUrl type
    fn from( url:BaseUrl ) -> RobotFileParser<'a> {
        RobotFileParser::<'a>::new( Url::from( url ) )
    }
}


#[cfg( feature = "sitemap_conversion" )]
impl From<BaseUrl> for Location {
    /// Wraps a ```BaseUrl``` into a ```Location```, one of the building blocks of the sitemap crate
    fn from( url:BaseUrl ) -> Location {
        Location::from( Url::from( url ) )
    }
}

#[cfg( feature = "sitemap_conversion" )]
impl From< BaseUrl> for SiteMapEntry {
    /// Conversion into a ```SiteMapEntry``` for a ```BaseUrl```
    fn from( url:BaseUrl ) -> SiteMapEntry {
        SiteMapEntry{
            loc: Location::from( url ),
            lastmod: LastMod::None,
        }
    }
}

#[cfg( feature = "sitemap_conversion" )]
impl From< BaseUrl > for UrlEntry {
    /// Conversion into a ```UrlEntry``` for a ```BaseUrl```
    fn from( url:BaseUrl ) -> UrlEntry {
        UrlEntry {
            loc: Location::from( url ),
            lastmod: LastMod::None,
            changefreq: ChangeFreq::None,
            priority: Priority::None,
        }
    }
}

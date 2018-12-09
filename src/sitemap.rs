
use BaseUrl;
use url::Url;

extern crate sitemap;
use self::sitemap::structs::{Location, SiteMapEntry, UrlEntry, LastMod, ChangeFreq, Priority};

impl From<BaseUrl> for Location {
    /// Wraps a ```BaseUrl``` into a ```Location```, one of the building blocks of the sitemap crate
    fn from( url:BaseUrl ) -> Location {
        Location::from( Url::from( url ) )
    }
}

impl From< BaseUrl> for SiteMapEntry {
    /// Conversion into a ```SiteMapEntry``` for a ```BaseUrl```
    fn from( url:BaseUrl ) -> SiteMapEntry {
        SiteMapEntry{
            loc: Location::from( url ),
            lastmod: LastMod::None,
        }
    }
}

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


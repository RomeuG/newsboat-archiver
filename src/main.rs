use sqlite;
use chrono::{Utc, TimeZone, DateTime};

#[derive(Debug)]
struct Feed {
    rssurl: Option<String>,
    url: Option<String>,
    title: Option<String>,
    lastmodified: Option<DateTime<Utc>>,
    is_rtl: Option<bool>,
    etag: Option<String>,
}

impl Feed {
    fn from_tuple_array(ta: &[(&str, Option<&str>)]) -> Feed {
        let feed = Feed {
            rssurl: ta[0].1.map(|s| s.to_string()),
            url: ta[1].1.map(|s| s.to_string()),
            title: ta[2].1.map(|s| s.to_string()),
            lastmodified: ta[3].1.map(|s| Utc.timestamp(s.parse::<i64>().unwrap(), 0)),
            is_rtl: ta[4].1.map(|s| s != "0"),
            etag: ta[5].1.map(|s| s.to_string()),
        };

        return feed;
    }
}

impl ToString for Feed {
    fn to_string(&self) -> String {
        let empty_str = "".to_owned();
        let empty_bool = false;
        let empty_date = Utc.timestamp(0,0);

        return format!("rssurl = \"{}\"\nurl = \"{}\"\ntitle = \"{}\"\nlastmodified = \"{}\"\nis_rtl = \"{}\"\netag = \"{}\"",
            self.rssurl.as_ref().unwrap_or_else(|| &empty_str),
            self.url.as_ref().unwrap_or_else(|| &empty_str),
            self.title.as_ref().unwrap_or_else(|| &empty_str),
            self.lastmodified.as_ref().unwrap_or_else(|| &empty_date),
            self.is_rtl.as_ref().unwrap_or_else(|| &empty_bool),
            self.etag.as_ref().unwrap_or_else(|| &empty_str),
        );
    }
}

fn main() {
    let connection = sqlite::open("/home/romeu/.local/share/newsboat/cache.db").unwrap();

    connection
        .iterate("SELECT * FROM rss_feed", |pairs| {
            let feed = Feed::from_tuple_array(pairs);
            println!("{}\n", feed.to_string());

            true
        })
        .unwrap();
}

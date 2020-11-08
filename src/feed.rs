use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug)]
pub struct Feed {
    pub rssurl: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub lastmodified: Option<DateTime<Utc>>,
    pub is_rtl: Option<bool>,
    pub etag: Option<String>,
}

impl Feed {
    pub fn from_tuple_array(ta: &[(&str, Option<&str>)]) -> Feed {
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
        return format!("{:?}", self);
    }
}

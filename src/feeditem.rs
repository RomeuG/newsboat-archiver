use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug, Clone)]
pub struct FeedItem {
    pub id: Option<u32>,
    pub guid: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub url: Option<String>,
    pub feedurl: Option<String>,
    pub pubdate: Option<DateTime<Utc>>,
    pub content: Option<String>,
    pub unread: Option<bool>,
    pub enclosure_url: Option<String>,
    pub enclosure_type: Option<String>,
    pub enqueued: Option<String>,
    pub flags: Option<String>,
    pub deleted: Option<bool>,
    pub base: Option<String>,
}

impl FeedItem {
    pub fn from_tuple_array(ta: &[(&str, Option<&str>)]) -> FeedItem {
        let feed_item = FeedItem {
            id: ta[0].1.map(|s| s.parse::<u32>().unwrap()),
            guid: ta[1].1.map(|s| s.to_string()),
            title: ta[2].1.map(|s| s.to_string()),
            author: ta[3].1.map(|s| s.to_string()),
            url: ta[4].1.map(|s| s.to_string()),
            feedurl: ta[5].1.map(|s| s.to_string()),
            pubdate: ta[6].1.map(|s| Utc.timestamp(s.parse::<i64>().unwrap(), 0)),
            content: ta[7].1.map(|s| s.to_string()),
            unread: ta[8].1.map(|s| s == "0"),
            enclosure_url: ta[9].1.map(|s| s.to_string()),
            enclosure_type: ta[10].1.map(|s| s.to_string()),
            enqueued: ta[11].1.map(|s| s.to_string()),
            flags: ta[12].1.map(|s| s.to_string()),
            deleted: ta[13].1.map(|s| s == "1"),
            base: ta[14].1.map(|s| s.to_string()),
        };

        return feed_item;
    }
}

impl ToString for FeedItem {
    fn to_string(&self) -> String {
        return format!("{:?}", self);
    }
}

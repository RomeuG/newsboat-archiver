use chrono::{DateTime, TimeZone, Utc};
use sqlite;

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
        let empty_date = Utc.timestamp(0, 0);

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
#[derive(Debug)]
struct FeedItem {
    id: Option<u32>,
    guid: Option<String>,
    title: Option<String>,
    author: Option<String>,
    url: Option<String>,
    feedurl: Option<String>,
    pubdate: Option<DateTime<Utc>>,
    content: Option<String>,
    unread: Option<bool>,
    enclosure_url: Option<String>,
    enclosure_type: Option<String>,
    enqueued: Option<String>,
    flags: Option<String>,
    deleted: Option<bool>,
    base: Option<String>,
}

impl FeedItem {
    fn from_tuple_array(ta: &[(&str, Option<&str>)]) -> FeedItem {
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

fn db_get_feed(dbcon: &sqlite::Connection) -> Vec<Feed> {
    let mut feed_list = vec![];

    dbcon
        .iterate("SELECT * FROM rss_feed", |pairs| {
            let feed = Feed::from_tuple_array(pairs);
            feed_list.push(feed);

            true
        })
        .unwrap();

    return feed_list;
}

fn db_get_feed_items(dbcon: &sqlite::Connection) -> Vec<FeedItem> {
    let mut feed_item_list = vec![];

    dbcon
        .iterate("SELECT * FROM rss_item", |pairs| {
            let feed = FeedItem::from_tuple_array(pairs);
            feed_item_list.push(feed);

            true
        })
        .unwrap();

    return feed_item_list;
}

fn main() {
    let connection = sqlite::open("/home/romeu/.local/share/newsboat/cache.db").unwrap();

    let feed_list = db_get_feed(&connection);
    let feed_item_list = db_get_feed_items(&connection);

    for item in feed_item_list {
        println!("{}\n", item.to_string());
    }
}

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
        return format!("{:?}", self);
    }
}
#[derive(Debug, Clone)]
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
            let feed_item = FeedItem::from_tuple_array(pairs);
            feed_item_list.push(feed_item);

            true
        })
        .unwrap();

    return feed_item_list;
}

// fn myfunc<'a>(mut strings: impl Iterator<Item=&'a str>, key: &'a str) -> bool {
//         strings.any(|item| key.contains(item))
// }

const URL_BLACKLIST: [&'static str; 2] = ["https://www.youtube.com/", "https://nitter.net/"];

fn is_url_in_blacklist<'a>(url: &'a str) -> bool {
    for blacklisted in &URL_BLACKLIST {
        if url.contains(blacklisted) {
            return true;
        }
    }

    return false;
}

fn str_sanitize(s: String) -> String {
    return s
        .replace(" ", "-")
        .replace(",", "")
        .replace(":", "")
        .replace("(", "")
        .replace(")", "")
        .replace("'", "")
        .replace("*", "");
}

pub trait StringExtensions {
    fn sanitize(&mut self) -> String;
}

impl StringExtensions for String {
    fn sanitize(&mut self) -> String {
        return self
            .replace(" ", "-")
            .replace(",", "")
            .replace(":", "")
            .replace("(", "")
            .replace(")", "")
            .replace("'", "")
            .replace("*", "")
            .replace("|", "")
            .replace(";", "")
            .replace("‘", "")
            .replace("`", "")
            .replace("`", "")
            .replace("…", "")
            .replace("...", "")
            .replace("<", "")
            .replace(">", "")
            .replace("&", "")
            .replace("--", "-")
            .replace("--", "-")
            .replace("--", "-");
    }
}

fn main() {
    let args = clap::App::new("newsboat-archiver")
        .version("1.0")
        .author("Romeu Vieira <romeu.bizz@gmail.com>")
        .about("Archive Newsboat DB information")
        .arg(
            clap::Arg::with_name("File")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Database file")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("Output")
                .short("d")
                .long("directory")
                .value_name("DIRECTORY")
                .help("Output directory")
                .takes_value(true),
        )
        .get_matches();

    let arg_db = args.value_of("File").expect("File argument not valid!");
    let arg_directory = args
        .value_of("Output")
        .expect("Directory argument is not valid!");

    let dir_metadata = std::fs::metadata(arg_directory).unwrap();
    if !dir_metadata.is_dir() {
        println!("Invalid path: not a directory!\n");
        std::process::exit(-1);
    }

    // e.g.: /home/romeu/.local/share/newsboat/cache.db
    let connection = sqlite::open(arg_db).unwrap();

    let feed_list = db_get_feed(&connection);
    let feed_item_list = db_get_feed_items(&connection);

    for feed in feed_list {
        let feed_url = feed.url.unwrap();
        let feed_rssurl = feed.rssurl.unwrap();
        let feed_title = feed.title.unwrap().sanitize();

        if is_url_in_blacklist(&feed_url) {
            continue;
        }

        // create directory
        let feed_dir = format!("{}/{}", arg_directory, feed_title);
        std::fs::create_dir(&feed_dir).expect("Directory could not be created.");

        let feed_item_list_clone = feed_item_list.clone();

        let items = feed_item_list_clone
            .iter()
            .filter(|item| {
                item.feedurl.clone().unwrap().contains(&feed_url)
                    || item.feedurl.clone().unwrap().contains(&feed_rssurl)
            })
            .collect::<Vec<_>>();

        for item in items {
            let title = item.title.clone().unwrap().sanitize();

            let url = item.url.as_ref().unwrap();
            let monolith = format!("monolith -s {} > {}/{}.html", url, feed_dir, title);

            println!("Command to execute: {}", monolith);

            // use `output()` to block
            std::process::Command::new("sh")
                .arg("-c")
                .arg(monolith)
                .output()
                .expect("Failed to execute monolith.");
        }
    }
}

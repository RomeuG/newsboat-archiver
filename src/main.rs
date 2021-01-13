mod feed;
mod feeditem;

use feed::*;
use feeditem::*;

use sqlite;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Default)]
struct Setting {
    cmd: String,
    url: String,
    args: String,
}

impl Setting {
    fn is_empty(&self) -> bool {
        return self.cmd.is_empty() && self.url.is_empty() && self.args.is_empty();
    }

    // fn empty() -> Setting {
    //     return Setting {
    //         cmd: "".to_string(),
    //         url: "".to_string(),
    //         args: "".to_string()
    //     }
    // }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_blacklist() -> Vec<String> {
    let mut list: Vec<String> = vec![];

    if let Ok(lines) = read_lines("blacklist.conf") {
        for line in lines {
            if let Ok(content) = line {
                list.push(content);
            }
        }
    }

    return list;
}

fn get_settings() -> Vec<Setting> {
    let mut list: Vec<Setting> = vec![];

    if let Ok(lines) = read_lines("settings.conf") {
        for line in lines {
            if let Ok(content) = line {
                let split = content.split("|").collect::<Vec<_>>();

                if split.len() == 3 {
                    let setting = Setting {
                        cmd: split[0].to_string(),
                        url: split[1].to_string(),
                        args: split[2].to_string(),
                    };
                    list.push(setting);
                }
            }
        }
    }

    return list;
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

fn is_url_in_blacklist<'a>(url: &'a str, list: &[String]) -> bool {
    for blacklisted in list {
        if url.contains(blacklisted) {
            return true;
        }
    }

    return false;
}

fn get_setting_from_url(url: String, list: &Vec<Setting>) -> Setting {
    for setting in list {
        if url.contains(&setting.url) {
            return setting.clone();
        }
    }

    // return Setting::empty();
    return Setting::default();
}

pub trait StringExtensions {
    fn sanitize(&mut self) -> String;
}

// TODO: try
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
        panic!("Invalid path: not a directory!\n");
    }

    // get lists
    let blacklist = get_blacklist();
    let settings = get_settings();

    // e.g.: /home/romeu/.local/share/newsboat/cache.db
    let connection = sqlite::open(arg_db).unwrap();

    let feed_list = db_get_feed(&connection);
    let feed_item_list = db_get_feed_items(&connection);

    for feed in feed_list {
        let feed_url = feed.url.unwrap();
        let feed_rssurl = feed.rssurl.unwrap();
        let feed_title = feed.title.unwrap().sanitize();

        if is_url_in_blacklist(&feed_url, &blacklist) {
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
            let setting = get_setting_from_url(url.clone(), &settings);

            println!("Setting found: {:?}", setting);

            let cmd: String;
            if setting.is_empty() {
                cmd = format!("monolith -s {} > {}/{}.html", url, feed_dir, title);
            } else {
                if setting.cmd == "monolith" {
                    cmd = format!(
                        "monolith -{} {} > {}/{}.html",
                        setting.args, url, feed_dir, title
                    );
                } else if setting.cmd == "lynx" {
                    cmd = format!("lynx {} -dump > {}/{}.txt", url, feed_dir, title);
                } else {
                    cmd = format!("monolith -s {} > {}/{}.html", url, feed_dir, title);
                }
            }

            println!("Command to execute: {}", cmd);

            // use `output()` to block
            std::process::Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()
                .expect("Failed to execute monolith.");
        }
    }
}

extern crate reqwest; // 0.9.18
extern crate roxmltree; // 0.13.5
extern crate dirs; // 4.0.0
extern crate chrono; // 0.4.23

use std::io::Read;
use std::fs::File;

#[derive(Ord, PartialOrd)]
struct FeedItem<'a> {
    feed: &'a FeedConfig,
    title : String,
    url: String,
    icon: String,
    timestamp: chrono::DateTime<chrono::Utc>
}

impl PartialEq for FeedItem<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}
impl Eq for FeedItem<'_> {}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct FeedConfig {
    name : String,
    url: String
}


impl FeedConfig {
    fn new(name: &str, url: &str) -> FeedConfig {
        FeedConfig {
            name: name.to_string(),
            url: url.to_string()
        }
    }

    fn create_item(&self, title: &str, url: &str, icon: &str, timestamp: &chrono::DateTime<chrono::Utc>) -> FeedItem {
        FeedItem {
            feed: &self,
            title: title.to_string(),
            url: url.to_string(),
            icon: icon.to_string(),
            timestamp: timestamp.to_owned()
        }
    }

    fn check(&self) -> Result<Vec<FeedItem>, Box<(dyn std::error::Error)>> {
        let mut res = reqwest::get(&self.url)?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
     
        let doc = roxmltree::Document::parse(body.as_str())?;
        let mut feed = doc.root().first_element_child().unwrap();
        let is_rss = feed.has_tag_name("rss");
        let mut items = Vec::new();
        let mut item_tag = "entry";
        let mut timestamp_tag = "updated";
        let mut icon_tag = "icon";
        if is_rss {
            //println!("Is RSS");
            feed = feed.first_element_child().unwrap();
            item_tag = "item";
            timestamp_tag = "pubDate";
            icon_tag = "image";
        }
        let mut icon = "";
        for feed_sub_node in feed.children() {
            if feed_sub_node.has_tag_name(icon_tag) {
                if is_rss{
                    icon = feed_sub_node.children().find(|x| x.has_tag_name("url")).unwrap().text().unwrap()
                } else {
                    icon = feed_sub_node.text().unwrap();
                }
            }
            if feed_sub_node.has_tag_name(item_tag) {
                let mut title = "";
                let mut url = "";
                let mut timestamp = chrono::DateTime::default();
                for item_sub_node in feed_sub_node.children() {
                    if item_sub_node.has_tag_name("title") {
                        title = item_sub_node.text().unwrap();
                    } else if item_sub_node.has_tag_name("link") {
                        if is_rss {
                            url = item_sub_node.text().unwrap();
                        } else if ! item_sub_node.has_attribute("rel") {
                            url = item_sub_node.attribute("href").unwrap();
                        }
                    } else if item_sub_node.has_tag_name(timestamp_tag) {
                        if is_rss {
                            timestamp = chrono::DateTime::parse_from_rfc2822(item_sub_node.text().unwrap()).unwrap().with_timezone(&chrono::Utc);
                        } else {
                            timestamp = chrono::DateTime::parse_from_rfc3339(item_sub_node.text().unwrap()).unwrap().with_timezone(&chrono::Utc);
                        }
                    }
                }
                let feed_item = self.create_item(title, url, icon, &timestamp);
                items.push(feed_item);
    
            }
        }
        // deduplicate based on url - leaving only the first occurence
        items.sort();
        items.dedup();
        
        items.sort_by(|a,b| b.timestamp.cmp(&a.timestamp));
        println!("Name: {} Count {}", &self.name, items.len());
        return Ok(items);
    }
    
}

fn main() {
    let feeds = read_opml().expect("unable to read feeds");
    let mut all_items = Vec::new();  
    for i in 0 .. feeds.len() {
        all_items.extend(feeds[i].check().expect("unable to check feed"));
    }
    println!("All count {}", all_items.len());
    all_items.sort_by(|a,b| b.timestamp.cmp(&a.timestamp));
    //all_items.reverse();
    for item in all_items {
        println!("{} {} {}", item.timestamp, item.feed.name, item.title);
    }
    
}

fn read_opml() -> Result<Vec<FeedConfig>, Box<(dyn std::error::Error)>> {
    let mut opml_file_path = dirs::data_local_dir().unwrap();
    opml_file_path = opml_file_path.join("kiss_rss.opml");
    let mut opml_file = File::open(opml_file_path)?;
    let mut opml_text = String::new();
    opml_file.read_to_string(&mut opml_text)?;
    let opml = roxmltree::Document::parse(opml_text.as_str())?;
    let outlines = opml.descendants().filter(|x| x.has_tag_name("outline"));
    let mut feed_configs = Vec::new();
    for outline in outlines {
        let feed_config = FeedConfig::new(
            outline.attribute("text").unwrap(),
            outline.attribute("xmlUrl").unwrap());
        feed_configs.push(feed_config);
    }
    return Ok(feed_configs);
}

 

extern crate reqwest; // 0.9.18
extern crate roxmltree; // 0.13.5
extern crate dirs; // 4.0.0
extern crate chrono; // 0.4.23

use std::io::Read;
use std::fs::File;
use std::thread;
use std::time;

#[derive(Ord, PartialOrd)]
struct NewsItem {
    subscription: String,
    icon: String,
    title : String,
    url: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    description: String
}

impl NewsItem {
    fn new(subscription: &str, icon: &str, title: &str, url: &str, timestamp: &chrono::DateTime<chrono::Utc>, description: &str) -> NewsItem {
        NewsItem { 
            subscription: subscription.to_string(),
            icon: icon.to_string(),
            title: title.to_string(), 
            url: url.to_string(), 
            timestamp: timestamp.to_owned(),
            description: description.to_string()
        }
    }
}

impl PartialEq for NewsItem {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}
impl Eq for NewsItem {}

struct Subscription {
    name : String,
    url: String
}

impl Subscription {
    fn new(name: &str, url: &str) -> Subscription {
        Subscription {
            name: name.to_string(),
            url: url.to_string()
        }
    }

    fn get_items(&self) -> Result<Vec<NewsItem>, Box<(dyn std::error::Error)>> {
        let mut items = Vec::new();
        let mut res = reqwest::get(&self.url)?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
     
        let doc = roxmltree::Document::parse(body.as_str())?;
        let mut channel = doc.root().first_element_child().ok_or("xml doc is missing child elements")?;
        let is_rss = channel.has_tag_name("rss");
        let mut item_tag = "entry";
        let mut timestamp_tag = "updated";
        let mut icon_tag = "icon";
        if is_rss {
            channel = channel.first_element_child().ok_or("rss element is missing channel element")?;
            item_tag = "item";
            timestamp_tag = "pubDate";
            icon_tag = "image";
        }
        let mut icon_elements = channel.children().filter(|x| x.has_tag_name(icon_tag));
        let icon = match icon_elements.next() {
            Some(node) => { 
                node.text().unwrap_or("")
            },
            None => { "" }
        };
        for item_node in channel.children().filter(|x| x.has_tag_name(item_tag)) {
            let mut title = "";
            let mut url = "";
            let mut timestamp = chrono::DateTime::default();
            let mut description = "";
            for item_sub_node in item_node.children() {
                if item_sub_node.has_tag_name("title") {
                    title = item_sub_node.text().unwrap_or("");
                } else if item_sub_node.has_tag_name("link") {
                    if is_rss{
                        url = item_sub_node.text().unwrap_or("");
                    } else if ! item_sub_node.has_attribute("rel") {
                        url = item_sub_node.attribute("href").unwrap_or("");
                    }
                } else if item_sub_node.has_tag_name(timestamp_tag) {
                    let timestamp_str = item_sub_node.text().unwrap_or("");
                    if is_rss{
                        timestamp = chrono::DateTime::parse_from_rfc2822(timestamp_str).unwrap_or_default().with_timezone(&chrono::Utc);
                    } else {
                        timestamp = chrono::DateTime::parse_from_rfc3339(timestamp_str).unwrap_or_default().with_timezone(&chrono::Utc);
                    }
                } else if  item_sub_node.has_tag_name("description") {
                    description = item_sub_node.text().unwrap_or("");
                }
            }
            let item = NewsItem::new(&self.name, icon, title, url, &timestamp, description);
            items.push(item);

        }
        // println!("Name: {} Count {}", &self.name, items.len());
        normalise(&mut items);
        Ok(items)
    }
    
}

fn read_opml() -> Result<Vec<Subscription>, Box<(dyn std::error::Error)>> {
    let mut opml_file_path = dirs::data_local_dir().ok_or("Unable to find local used dir")?;
    opml_file_path = opml_file_path.join("kiss_rss.opml");
    let mut opml_file = File::open(opml_file_path)?;
    let mut opml_text = String::new();
    opml_file.read_to_string(&mut opml_text)?;
    let opml = roxmltree::Document::parse(opml_text.as_str())?;
    let outlines = opml.descendants().filter(|x| x.has_tag_name("outline"));
    let mut subscriptions = Vec::new();
    for outline in outlines {
        let name =  outline.attribute("text").unwrap_or("");
        let url = outline.attribute("xmlUrl").unwrap_or("");
        if name != "" && url != "" {
            subscriptions.push(Subscription::new( name, url));
        }
    }
    Ok(subscriptions)
}

fn normalise(items: &mut Vec<NewsItem>) {
    items.sort();
    items.dedup();
    items.sort_by(|a,b| b.timestamp.cmp(&a.timestamp));
}

fn refresh() -> Result<Vec<NewsItem>, Box<(dyn std::error::Error)>> {
    let subscriptions = read_opml()?;
    let mut items = Vec::new();
    for subscription in subscriptions {
        let sub_items = subscription.get_items()?;
        items.extend(sub_items);
    }
    normalise(&mut items);
    Ok(items)

}

fn main() {
    for i in 0 .. 3 {
        println!("{} -----------------", i);
        let mut items = refresh().expect("refresh failed"); 
        items.reverse();
        for item in items {
            println!("{} {} {}", item.timestamp, item.subscription, item.title);
        }
        println!("--------------------\n");
        thread::sleep(time::Duration::from_secs(60));

    }
}

 

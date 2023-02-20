extern crate reqwest; // 0.9.18
extern crate roxmltree; // 0.13.5
extern crate dirs; // 4.0.0
extern crate chrono; // 0.4.23

use std::io::Read;
use std::fs::File;

#[derive(Ord, PartialOrd, Clone)]
pub struct NewsItem {
    pub subscription: String,
    pub title : String,
    pub url: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub summary: String
}

impl NewsItem {
    fn new(subscription: &str, title: &str, url: &str, timestamp: &chrono::DateTime<chrono::Utc>, summary: &str) -> NewsItem {
        NewsItem { 
            subscription: subscription.to_string(),
            title: title.to_string(), 
            url: url.to_string(), 
            timestamp: timestamp.to_owned(),
            summary: summary.to_string()
        }
    }
}

impl PartialEq for NewsItem {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}
impl Eq for NewsItem {}

pub struct Subscription {
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

    fn sync(&self) -> Result<Vec<NewsItem>, Box<(dyn std::error::Error)>> {
        let rss = self.download()?;
        self.parse(&rss)
    }

    fn download(&self) -> Result<String, Box<(dyn std::error::Error)>> {
        let mut res = reqwest::get(&self.url)?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        Ok(body)
    } 

    fn parse(&self, rss: &str) -> Result<Vec<NewsItem>, Box<(dyn std::error::Error)>> {
        let mut items = Vec::new();
        let doc = roxmltree::Document::parse(rss)?;
        let mut channel = doc.root().first_element_child().ok_or("xml doc is missing child elements")?;
        let is_rss = channel.has_tag_name("rss");
        let mut item_tag = "entry";
        let mut timestamp_tag = "updated";
        let mut summary_tag  = "content";
        if is_rss {
            channel = channel.first_element_child().ok_or("rss element is missing channel element")?;
            item_tag = "item";
            timestamp_tag = "pubDate";
            summary_tag = "description";        }
        for item_node in channel.children().filter(|x| x.has_tag_name(item_tag)) {
            let mut title = "";
            let mut url = "";
            let mut timestamp = chrono::DateTime::default();
            let mut summary = "";
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
                } else if  item_sub_node.has_tag_name(summary_tag) {
                    summary = item_sub_node.text().unwrap_or("");
                }
            }
            let item = NewsItem::new(&self.name, title, url, &timestamp, summary);
            items.push(item);

        }
        // println!("Name: {} Count {}", &self.name, items.len());
        normalise(&mut items);
        Ok(items)
    }
    
}

struct SubscriptionSet {
    subscriptions: Vec<Subscription>
}

impl SubscriptionSet {

    fn new() -> SubscriptionSet {
        SubscriptionSet { subscriptions: Vec::new() }
    }

    fn load_from_opml(&mut self) -> Result<(), Box<(dyn std::error::Error)>> {
        let mut opml_file_path = dirs::data_local_dir().ok_or("Unable to find local used dir")?;
        opml_file_path = opml_file_path.join("kiss_rss.opml");
        let mut opml_file = File::open(opml_file_path)?;
        let mut opml_text = String::new();
        opml_file.read_to_string(&mut opml_text)?;
        let opml = roxmltree::Document::parse(opml_text.as_str())?;
        let outlines = opml.descendants().filter(|x| x.has_tag_name("outline"));
        self.subscriptions = Vec::new();
        for outline in outlines {
            let name =  outline.attribute("text").unwrap_or("");
            let url = outline.attribute("xmlUrl").unwrap_or("");
            if name != "" && url != "" {
                self.subscriptions.push(Subscription::new( name, url));
            }
        }
        Ok(())
    }
}

fn normalise(items: &mut Vec<NewsItem>) {
    items.sort();
    items.dedup();
    items.sort_by(|a,b| b.timestamp.cmp(&a.timestamp));
}

pub async fn refresh_async() -> Result<Vec<NewsItem>, Box<(dyn std::error::Error)>> {
    refresh()
}

pub fn refresh() -> Result<Vec<NewsItem>, Box<(dyn std::error::Error)>> {
    let mut set = SubscriptionSet::new();
    set.load_from_opml()?;
    let mut items = Vec::new();
    for subscription in set.subscriptions {
        let sub_items = subscription.sync()?;
        items.extend(sub_items);
    }
    normalise(&mut items);
    Ok(items)

}

 

use std::io::Read;
use std::fs::OpenOptions;
use std::fmt;

use chrono::{SubsecRound};
use xml_builder::{XMLBuilder, XMLElement, XMLVersion};

use crate::news::{ NewsItem, NewsItemList };

#[derive(Clone, Debug, Default)]
pub enum Status {
    #[default]
    Unknown,
    Ok,
    DownloadFailed(String),
    ParseFailed(String)
}

impl Status {
    pub fn get_message(&self) -> String {
        match self {
            Status::Unknown => "".to_string(),
            Status::Ok => "".to_string(),
            Status::DownloadFailed(s) => s.to_string(),
            Status::ParseFailed(s) => s.to_string()
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Unknown => write!(f, "?"),
            Status::Ok => write!(f, "✓"),
            Status::DownloadFailed(_) => write!(f, "✕"),
            Status::ParseFailed(_) => write!(f, "✕")
        }
    }
}

#[derive(Clone)]
pub struct Subscription {
    pub name : String,
    pub url: String,
    pub status: Status,
    pub last_sync: chrono::DateTime<chrono::Utc>,
    pub update_rate: f32,
    pub item_list: NewsItemList
}

impl Subscription {
    pub fn from_url(url: &str) -> Option<Subscription> {
        if url.len() > 0 {
            Some(
                Subscription {
                    name: "".to_string(),
                    url: url.to_string(),
                    status: Status::default(),
                    last_sync: chrono::DateTime::default(),
                    update_rate: 0.0,
                    item_list: NewsItemList::new()
                }
            )
        } else { None }
    }

    // pub fn name(&self) -> String { self.name.to_owned() }
    // pub fn url(&self) -> String { self.url.to_owned() }
    // pub fn status(&self) -> String { self.status.to_string() }
    // pub fn status_message(&self) -> String { self.status.get_message() }
    // pub fn last_sync(&self) -> chrono::DateTime<chrono::Utc> { self.last_sync }
    // pub fn update_rate(&self) -> f32 { self.update_rate }
    // pub fn items(&self) -> NewsItemList { self.item_list.clone() }

    pub fn set_name(&mut self, name: &str) {
        if name.len() > 0 {
            self.name = name.to_owned();
        }
    }

    pub fn sync(&mut self) {
        self.item_list = NewsItemList::new();
        match self.download() {
            Ok(downloaded) => {
                match self.parse(&downloaded) {
                    Ok(parsed) => {
                        self.status = Status::Ok;
                        self.last_sync = chrono::offset::Utc::now().round_subsecs(0);
                        self.item_list = parsed;
                    }
                    Err(e) => self.status = Status::ParseFailed(e.to_string())
                }
            }
            Err(e) => self.status = Status::DownloadFailed(e.to_string())
        };
    }

    fn download(&self) -> Result<String, Box<(dyn std::error::Error)>> {
        let mut res = reqwest::get(&self.url)?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        Ok(body)
    } 

    fn parse(&mut self, rss: &str) -> Result<NewsItemList, Box<(dyn std::error::Error)>> {
        let mut item_list = NewsItemList::new();
        let doc = roxmltree::Document::parse(rss)?;
        let mut channel = doc.root().first_element_child().ok_or("xml doc is missing child elements")?;
        let is_rss = channel.has_tag_name("rss");
        let name_tag = "title";
        let mut item_tag = "entry";
        let mut timestamp_tag = "updated";
        let mut summary_tag  = "content";
        if is_rss {
            channel = channel.first_element_child().ok_or("rss element is missing channel element")?;
            item_tag = "item";
            timestamp_tag = "pubDate";
            summary_tag = "description";        
        }
        if self.name.len() == 0 {
            let name = match channel.children().find(|x| x.has_tag_name(name_tag)) {
                Some(node) => node.text().unwrap_or(""),
                None => ""
            };
            self.set_name(name)
        }
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
            if (chrono::offset::Utc::now() - timestamp).num_days() < 3 {
                // only keep items from the last two daya
                let item = NewsItem::new(&self.name, title, url, &timestamp, summary);
                item_list.push(item);
            }

        }
        // println!("Name: {} Count {}", &self.name, items.len());
        item_list.normalise();
        if item_list.len() > 0 {
            let earliest = item_list.last().unwrap().timestamp;
            //println!("{} {} {}", self.name, earliest, item_list.last().unwrap().title);
            let duration = chrono::offset::Utc::now() - earliest;
            self.update_rate = (item_list.len() as f32 / duration.num_seconds() as f32) *60.0 * 60.0;
        }
        Ok(item_list)
    }
    
}


#[derive(Clone)]
pub struct SubscriptionSet {
    pub opml_file_name: String,
    pub subscriptions: Vec<Subscription>
}

impl SubscriptionSet {

    pub fn new() -> SubscriptionSet {
        SubscriptionSet { 
            opml_file_name: "kiss_rss.opml".to_string(),
            subscriptions: Vec::new()
        }
    }

    pub fn load(&mut self) -> Result<(), Box<(dyn std::error::Error)>> {
        let mut opml_file_path = dirs::data_local_dir().ok_or("Unable to find local used dir")?;
        opml_file_path = opml_file_path.join(&self.opml_file_name);
        let mut opml_file = OpenOptions::new().read(true).open(opml_file_path)?;
        let mut opml_text = String::new();
        opml_file.read_to_string(&mut opml_text)?;
        let opml = roxmltree::Document::parse(opml_text.as_str())?;
        let outlines = opml.descendants().filter(|x| x.has_tag_name("outline"));
        self.subscriptions = Vec::new();
        for outline in outlines {
            if let Some(url) = outline.attribute("xmlUrl") {
                if url.len() > 0 {
                    if let Some(subscription) = &mut Subscription::from_url(url) {
                        subscription.set_name(outline.attribute("text").unwrap_or(""));
                        self.add(subscription);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<(dyn std::error::Error)>> {
        let mut xml = XMLBuilder::new()
        .version(XMLVersion::XML1_1)
        .encoding("UTF-8".into())
        .build();

        let mut opml = XMLElement::new("opml");
        opml.add_attribute("version", "1.0");
        let mut head = XMLElement::new("head");
        let mut title = XMLElement::new("title");
        title.add_text("Kiss RSS".to_string()).unwrap();
        head.add_child(title).unwrap();
        opml.add_child(head).unwrap();
        let mut body = XMLElement::new("body");
        for subscription in &self.subscriptions {
            let mut outline = XMLElement::new("outline");
            outline.add_attribute("text", subscription.name.as_str());
            outline.add_attribute("type", "rss");
            outline.add_attribute("xmlUrl", subscription.url.as_str());
            body.add_child(outline).unwrap();
        }
        opml.add_child(body).unwrap();
        xml.set_root_element(opml);

        // let mut writer: Vec<u8> = Vec::new();
        // xml.generate(&mut writer).unwrap();
        // let xml_str = String::from_utf8(writer).unwrap();
        // println!("{}",xml_str);

        let mut opml_file_path = dirs::data_local_dir().ok_or("Unable to find local used dir")?;
        opml_file_path = opml_file_path.join(&self.opml_file_name);
        let opml_file = OpenOptions::new().write(true).truncate(true).open(opml_file_path)?;
        xml.generate(opml_file).unwrap();
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.subscriptions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.subscriptions.is_empty()
    }

    pub fn add(&mut self, subscrition: &Subscription) {
        self.subscriptions.push(subscrition.to_owned());
    }

    pub fn add_from_url(&mut self, url: &str) {
        if let Some(subscription) = &Subscription::from_url(url) {
            self.add(subscription);
        }
    }

    pub fn sync_all(&mut self) {
        for subscription in &mut self.subscriptions {
            subscription.sync();
        }
    }

    pub fn sync(&mut self, url: &str)  {
        for subscription in &mut self.subscriptions {
            if subscription.url == url {
                subscription.sync();
            }
        }
    }

    pub fn get_items(&self) -> NewsItemList {
        let mut item_list = NewsItemList::new();
        for subscription in &self.subscriptions {
            item_list.extend(&subscription.item_list);
        }
        item_list.normalise();
        item_list
    }
}

pub struct SubscriptionSetIter<'a> {
    subcription_set: &'a SubscriptionSet,
    i: usize,
}

impl<'a> Iterator for SubscriptionSetIter<'a> {
    type Item = &'a Subscription;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.subcription_set.subscriptions.len() {
            None
        } else {
            self.i += 1;
            Some(&self.subcription_set.subscriptions[self.i - 1])
        }
    }
}

impl<'a> IntoIterator for &'a SubscriptionSet {
    type Item = &'a Subscription;
    type IntoIter = SubscriptionSetIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        SubscriptionSetIter {
            subcription_set: self,
            i: 0,
        }
    }
}

// impl IntoIterator for SubscriptionSet {
//     type Item = Subscription;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.subscriptions.iter()
//     }
// }
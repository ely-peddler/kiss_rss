use std::io::Read;
use std::fs::File;

use chrono::SubsecRound;

use crate::news::{ NewsItem, NewsItemList };

#[derive(Clone)]
pub struct Subscription {
    pub name : String,
    pub url: String,
    pub status: String,
    pub last_sync: chrono::DateTime<chrono::Utc>,
    pub update_rate: f32
}

impl Subscription {
    fn new(name: &str, url: &str) -> Subscription {
        Subscription {
            name: name.to_string(),
            url: url.to_string(),
            status: "Unknown".to_string(),
            last_sync: chrono::DateTime::default(),
            update_rate: 0.0
        }
    }
    pub fn sync(&mut self) -> NewsItemList {
        let ret = NewsItemList::new();
        let rss = match self.download() {
            Ok(downloaded) => downloaded,
            Err(_) => {
                self.status = "Download failed".to_string();
                return ret;
            }
        };
        match self.parse(&rss) {
            Ok(parsed) => {
                self.status = "OK".to_string();
                self.last_sync = chrono::offset::Utc::now().round_subsecs(0);
                parsed
            },
            Err(_) => {
                self.status = "Parse failed".to_string();
                ret
            }
        }
        //return ret;
        
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

    pub fn sync(&mut self) -> NewsItemList {
        let mut item_list = NewsItemList::new();
        for subscription in &mut self.subscriptions {
            item_list.extend(subscription.sync());
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
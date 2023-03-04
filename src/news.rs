#[derive(Ord, PartialOrd, Clone)]
pub struct NewsItem {
    pub source: String,
    pub title : String,
    pub url: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub summary: String
}

impl NewsItem {
    pub fn new(source: &str, title: &str, url: &str, timestamp: &chrono::DateTime<chrono::Utc>, summary: &str) -> NewsItem {
        NewsItem { 
            source: source.to_string(),
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

#[derive(Clone)]
pub struct NewsItemList {
    pub items: Vec<NewsItem>
}

impl NewsItemList {
    pub fn new() -> NewsItemList {
        NewsItemList { items: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn push(&mut self, item: NewsItem) {
        self.items.push(item);
    }

    pub fn extend(&mut self, other: &NewsItemList) {
        self.items.extend(other.items.to_owned());
    }

    pub fn normalise(&mut self) {
        self.items.sort();
        self.items.dedup();
        self.items.sort_by(|a,b| b.timestamp.cmp(&a.timestamp));  
    }

    pub fn truncate(&mut self, len: usize) {
        self.items.truncate(len)
    }

    pub fn last(&self) -> Option<&NewsItem> {
        self.items.last()
    }
}

pub struct NewsItemListIter<'a> {
    item_list: &'a NewsItemList,
    i: usize,
}

impl<'a> Iterator for NewsItemListIter<'a> {
    type Item = &'a NewsItem;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.item_list.items.len() {
            None
        } else {
            self.i += 1;
            Some(&self.item_list.items[self.i - 1])
        }
    }
}

impl<'a> IntoIterator for &'a NewsItemList {
    type Item = &'a NewsItem;
    type IntoIter = NewsItemListIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        NewsItemListIter {
            item_list: self,
            i: 0,
        }
    }
}

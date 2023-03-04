use kiss_rss::sources::SourceList;

use std::{ thread, time };

fn main() {
    let mut sources = SourceList::new();
    sources.load().expect("Unable to load sources."); 
    sources.sync_all();
    let _item_list = sources.get_items();
    for source in &sources {
        println!("{} {}\t{}\t{}\t{}\t{}", source.status, source.update_rate, source.last_sync, source.update_rate, source.name, source.url);
    }
    thread::sleep(time::Duration::from_secs(10));
    sources.sync("https://www.theguardian.com/uk/rs");
    for source in &sources {
        println!("{} {}\t{}\t{}\t{}\t{}", source.status, source.update_rate, source.last_sync, source.update_rate, source.name, source.url);
    }
    sources.save().expect("Unable to save sources."); 

}

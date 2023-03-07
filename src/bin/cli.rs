use kiss_rss::sources::{SourceList, Status};

fn main() {
    let mut sources = SourceList::new();
    sources.load_from_user_file().expect("Unable to load_from_user_file sources."); 
    sources.sync_all();
    let _item_list = sources.get_items();
    for source in &sources {
        if !matches!(source.status, Status::Ok) || source.update_rate == 0.0 {
            println!("{} {}\t{}\t{}\t{}\t{}\t{}", source.status, source.status.get_message(), source.format, source.last_sync, source.update_rate, source.name, source.url);
        }
    }

}

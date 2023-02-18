
use kiss_rss;

fn main() {
    let mut items = kiss_rss::refresh().expect("refresh failed"); 
    items.reverse();
    for item in items {
        println!("{} | {}", item.timestamp, item.subscription);
        println!(" {}", item.title);
    }
}

 

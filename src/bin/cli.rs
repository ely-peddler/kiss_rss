use kiss_rss::subscriptions::SubscriptionSet;

use std::{ thread, time };

fn main() {
    let mut subscriptions = SubscriptionSet::new();
    subscriptions.load().expect("Unable to load subscriptions."); 
    subscriptions.sync_all();
    let _item_list = subscriptions.get_items();
    for subscription in &subscriptions {
        println!("{} {}\t{}\t{}\t{}\t{}", subscription.status, subscription.update_rate, subscription.last_sync, subscription.update_rate, subscription.name, subscription.url);
    }
    thread::sleep(time::Duration::from_secs(10));
    subscriptions.sync("https://www.theguardian.com/uk/rs");
    for subscription in &subscriptions {
        println!("{} {}\t{}\t{}\t{}\t{}", subscription.status, subscription.update_rate, subscription.last_sync, subscription.update_rate, subscription.name, subscription.url);
    }
    subscriptions.save().expect("Unable to save subscriptions."); 

}

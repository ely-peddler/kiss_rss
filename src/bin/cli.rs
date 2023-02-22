use kiss_rss::subscriptions::SubscriptionSet;

fn main() {
    let mut subscriptions = SubscriptionSet::new();
    subscriptions.load().expect("Unalve to load subscriptions."); 
    let _item_list= subscriptions.sync();
    for subscription in &subscriptions {
        println!("{}\t{}\t{}\t{}", subscription.status, subscription.last_sync, subscription.update_rate, subscription.name);
    }
}

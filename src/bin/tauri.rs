#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{ Mutex };

use kiss_rss::subscriptions::SubscriptionSet;
use scraper::{ Html, Node::Text };

fn get_short_summary(html_summary: &str, len: usize) -> String {
    let mut summary = String::new();
    let fragment = Html::parse_fragment(html_summary);
    for element in fragment.tree.values() {
        let text = match element {
            Text(element) =>  &**element,
            _ => ""
        };
        summary += text;
        if summary.len() > len {
            return summary
        }
    }
    summary
}

// fn get_feed_html(name: &str, item_list: &NewsItemList) -> String {
//     let mut html = format!("<div class=\"tab\" id=\"tab-feed-{}\">", name);
//     for item in item_list {
//         html += "<div class=\"news_item\">";
//         html += &format!("<div class=\"subscription_name\">{}</div>", item.subscription);
//         html += &format!("<div class=\"timestamp\">{}</div>", item.timestamp);
//         html += &format!("<div class=\"title\" onclick=openPage(\"{}\")>{}</div>", item.url, item.title);
//         html += &format!("<div class=\"summary\">{}</div>", get_short_summary(&item.summary, 100));
//         html += "</div>";
//     }
//     html += "</div>";
//     html
// }

// #[tauri::command]
// fn refresh_feeds(state: tauri::State<LockedSubscriptionSet>) -> String {
//     let mut mutex_gd = state.0.lock().unwrap();
//     let subscription_set = mutex_gd.as_mut().unwrap();
//     let latest_count = 40;
//     let item_list = subscription_set.sync();
//     let mut html = get_feed_html("all", &item_list);
//     let mut latest = item_list.clone();
//     latest.truncate(latest_count);
//     html += get_feed_html("latest", &latest).as_str();
//     html 
// }

// #[tauri::command]
// fn get_subscriptions_html(state: tauri::State<LockedSubscriptionSet>) -> String {
//     let mutex_gd = state.0.lock().unwrap();
//     let subscription_set = mutex_gd.as_ref().unwrap();
//     let mut html = String::new();
//     html += "<div class=\"subscription header\">";
//     html += "<div class=\"name\">Name</div>";
//     html += "<div class=\"timestamp\">Last Sync Time</div>";
//     html += "<div class=\"update_rate\">Updates</div>";
//     html += "<div class=\"status\">OK</div>";
//     html += "</div>";
//     for subscription in subscription_set {
//         html += "<div class=\"subscription\">";
//         html += &format!("<div class=\"name\">{}</div>", subscription.name);
//         html += &format!("<div class=\"timestamp\">{}</div>", subscription.last_sync);
//         html += &format!("<div class=\"update_rate\">{:.0} / day</div>", subscription.update_rate*24.0);
//         html += &format!("<div class=\"status tooltip\">{}<span class=\"tooltiptext\">{}</span></div>", if subscription.status == "OK" { "✔" } else { "✖" }, subscription.status);
//         html += &format!("<div class=\"url\">{}</div>", subscription.url);
//         html += "</div>";
//     }
//     html += "</div>";
//     html
// }

#[tauri::command]
fn load_subscriptions(state: tauri::State<LockedSubscriptionSet>) -> bool {
    let mut mutex_gd = state.0.lock().unwrap();
    let subscription_set = mutex_gd.as_mut().unwrap();
    match subscription_set.load() {
        Ok(()) => true,
        Err(e) => {
            println!("Problem loading the subscriptions: {}",e);
            false
        }
    }
}

#[tauri::command]
fn add_subscription(state: tauri::State<LockedSubscriptionSet>, url: String) {
    let mut mutex_gd = state.0.lock().unwrap();
    let subscription_set = mutex_gd.as_mut().unwrap();
    subscription_set.add_from_url(&url);
    subscription_set.save().unwrap_or_else(|error| {
        println!("Problem saving the subscriptions: {:?}", error);
    });
}

#[tauri::command]
fn sync_subscription(state: tauri::State<LockedSubscriptionSet>, url: String) {
    let mut mutex_gd = state.0.lock().unwrap();
    let subscription_set = mutex_gd.as_mut().unwrap();
    subscription_set.sync(&url);
}

#[tauri::command]
fn sync_all_subscriptions(state: tauri::State<LockedSubscriptionSet>) {
    let mut mutex_gd = state.0.lock().unwrap();
    let subscription_set = mutex_gd.as_mut().unwrap();
    subscription_set.sync_all();
}

#[tauri::command]
fn get_subscriptions_table(state: tauri::State<LockedSubscriptionSet>) -> String {
    let mutex_gd = state.0.lock().unwrap();
    let subscription_set = mutex_gd.as_ref().unwrap();
    let mut html = String::new();
    html += "<div class=\"subscription\">";
    html += "<div class=\"header\">";
    html += "<div class=\"name\">Name</div>";
    html += "<div class=\"timestamp\">Last Sync</div>";
    html += "<div class=\"update_rate\">Updates</div>";
    html += "<div class=\"status\">OK</div>";
    html += "</div></div>";
    for subscription in subscription_set {
        html += "<div class=\"subscription\">";
        html += "<div class=\"info\">";
        html += &format!("<div class=\"name\">{}</div>", subscription.name);
        html += &format!("<div class=\"timestamp\">{}</div>", subscription.last_sync);
        html += &format!("<div class=\"update_rate\">{:.0} / day</div>", subscription.update_rate*24.0);
        html += &format!("<div class=\"status\">{}</div>", subscription.status);
        html += "</div>";
        // html += &format!("<div class=\"url\">{}</div>", subscription.url);
        html += "</div>";
    }
    html += "</div>";
    html
}

#[tauri::command]
fn get_items(state: tauri::State<LockedSubscriptionSet>) -> String {
    let mutex_gd = state.0.lock().unwrap();
    let subscription_set = mutex_gd.as_ref().unwrap();
    let item_list = subscription_set.get_items();
    let mut html = String::new();
    for item in &item_list {
        html += "<div class=\"news_item\">";
        html += &format!("<div class=\"subscription_name\">{}</div>", item.subscription);
        html += &format!("<div class=\"timestamp\">{}</div>", item.timestamp);
        html += &format!("<div class=\"title\" onclick=openPage(\"{}\")>{}</div>", item.url, item.title);
        html += &format!("<div class=\"summary\">{}</div>", get_short_summary(&item.summary, 100));
        html += "</div>";
    }
    html
}

struct LockedSubscriptionSet(Mutex<Option<SubscriptionSet>>);
    
fn main() {
    let locked_subs = LockedSubscriptionSet(Mutex::new(Some(SubscriptionSet::new())));
    tauri::Builder::default()
        .manage(locked_subs)
        .invoke_handler(tauri::generate_handler![
            load_subscriptions, 
            add_subscription,
            sync_subscription, 
            sync_all_subscriptions,
            get_subscriptions_table,
            get_items
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

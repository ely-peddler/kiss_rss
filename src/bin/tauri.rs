#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use kiss_rss::NewsItem;

fn create_feed_tab(name: &str, items: &Vec<NewsItem>) -> String {
    let mut html = format!("<div class=\"tab\" id=\"tab-feed-{}\">", name);
    for item in items {
        let icon = url::Url::parse(&item.url).unwrap();
        html += "<div class=\"kiss_rss-news_item\">";
        html += &format!("<div class=\"kiss_rss-subscription\"><img class=\"kiss_rss-icon\" src=\"http://{}/favicon.ico\">{}</div>",icon.host_str().unwrap_or(""), item.subscription);
        html += &format!("<div class=\"kiss_rss-timestamp\">{}</div>", item.timestamp);
        html += &format!("<div class=\"kiss_rss-title\"><a href=\"{}\">{}</a></div>", item.url, item.title);
        html += "</div>";
    }
    html += "</div>";
    html
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn refresh() -> String {
    let result = kiss_rss::refresh();
    //let items = kiss_rss::refresh().unwrap();
    let inner_html = match result {
        Ok(items) => {
            let mut html = create_feed_tab("all", &items);
            let mut latest = items.clone();
            latest.truncate(20);
            html += create_feed_tab("latest", &latest).as_str();
            html },
        Err(_) => { "<div class=\"kiss_rss-news_item\">Loading RSS failed</div>".to_string() }
        //None => { "<div class=\"kiss_rss-news_item\">Loading RSS ...</div>".to_string() }
        };
    inner_html
}
    

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![refresh])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use kiss_rss::NewsItem;
use scraper::{Html, Node::Text};

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

fn create_feed_tab(name: &str, items: &Vec<NewsItem>) -> String {
    let mut html = format!("<div class=\"tab\" id=\"tab-feed-{}\">", name);
    for item in items {
        html += "<div class=\"kiss_rss-news_item\">";
        html += &format!("<div class=\"kiss_rss-subscription\">{}</div>", item.subscription);
        html += &format!("<div class=\"kiss_rss-timestamp\">{}</div>", item.timestamp);
        html += &format!("<div class=\"kiss_rss-title\" onclick=openPage(\"{}\")>{}</div>", item.url, item.title);
        html += &format!("<div class=\"kiss_rss-summary\">{}</div>", get_short_summary(&item.summary, 100));
        html += "</div>";
    }
    html += "</div>";
    html
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn refresh() -> String {
    let result = kiss_rss::refresh();
    let inner_html = match result {
        Ok(items) => {
            let mut html = create_feed_tab("all", &items);
            let mut latest = items.clone();
            latest.truncate(20);
            html += create_feed_tab("latest", &latest).as_str();
            html },
        Err(_) => { "<div class=\"kiss_rss-news_item\">Loading RSS failed</div>".to_string() }
        };
    inner_html
}
    
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![refresh])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

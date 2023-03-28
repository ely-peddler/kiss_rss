#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::sync::Mutex;

use kiss_rss::sources::SourceList;
use scraper::{ Html, Node::Text };

use readable_readability::Readability;
use url::Url;

#[tauri::command]
fn get_readable(url: &str) -> String {
    let html = match reqwest::blocking::get(url) {
        Ok(resp) => match resp.text() {
            Ok(html) => html.to_string(),
            Err(_) => String::new()
        },
        Err(_) => String::new()
    };
    let mut r = Readability::new();
    r.clean_conditionally(false);
    let (actual_tree, _actual_meta) = r.base_url(Url::parse(url).unwrap()).parse(&html);
    actual_tree.totring()
}


#[tauri::command]
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
//         html += &format!("<div class=\"source_name\">{}</div>", item.source);
//         html += &format!("<div class=\"timestamp\">{}</div>", item.timestamp);
//         html += &format!("<div class=\"title\" onclick=openPage(\"{}\")>{}</div>", item.url, item.title);
//         html += &format!("<div class=\"summary\">{}</div>", get_short_summary(&item.summary, 100));
//         html += "</div>";
//     }
//     html += "</div>";
//     html
// }

// #[tauri::command]
// fn refresh_feeds(state: tauri::State<LockedSourceList>) -> String {
//     let mut mutex_gd = state.0.lock().unwrap();
//     let source_list = mutex_gd.as_mut().unwrap();
//     let latest_count = 40;
//     let item_list = source_list.sync();
//     let mut html = get_feed_html("all", &item_list);
//     let mut latest = item_list.clone();
//     latest.truncate(latest_count);
//     html += get_feed_html("latest", &latest).as_str();
//     html 
// }

// #[tauri::command]
// fn get_sources_html(state: tauri::State<LockedSourceList>) -> String {
//     let mutex_gd = state.0.lock().unwrap();
//     let source_list = mutex_gd.as_ref().unwrap();
//     let mut html = String::new();
//     html += "<div class=\"source header\">";
//     html += "<div class=\"name\">Name</div>";
//     html += "<div class=\"timestamp\">Last Sync Time</div>";
//     html += "<div class=\"update_rate\">Updates</div>";
//     html += "<div class=\"status\">OK</div>";
//     html += "</div>";
//     for source in source_list {
//         html += "<div class=\"source\">";
//         html += &format!("<div class=\"name\">{}</div>", source.name);
//         html += &format!("<div class=\"timestamp\">{}</div>", source.last_sync);
//         html += &format!("<div class=\"update_rate\">{:.0} / day</div>", source.update_rate*24.0);
//         html += &format!("<div class=\"status tooltip\">{}<span class=\"tooltiptext\">{}</span></div>", if source.status == "OK" { "✔" } else { "✖" }, source.status);
//         html += &format!("<div class=\"url\">{}</div>", source.url);
//         html += "</div>";
//     }
//     html += "</div>";
//     html
// }

#[tauri::command]
fn load_user_sources(state: tauri::State<LockedSourceList>) -> bool {
    let mut mutex_gd = state.0.lock().unwrap();
    let source_list = mutex_gd.as_mut().unwrap();
    match source_list.load_from_user_file() {
        Ok(()) => true,
        Err(e) => {
            println!("Problem loading the sources: {}",e);
            false
        }
    }
}

#[tauri::command]
fn add_source(state: tauri::State<LockedSourceList>, url: String) {
    let mut mutex_gd = state.0.lock().unwrap();
    let source_list = mutex_gd.as_mut().unwrap();
    source_list.add_from_url(&url);
    source_list.save().unwrap_or_else(|error| {
        println!("Problem saving the sources: {:?}", error);
    });
}

#[tauri::command]
fn remove_source(state: tauri::State<LockedSourceList>, url: String) {
    let mut mutex_gd = state.0.lock().unwrap();
    let source_list = mutex_gd.as_mut().unwrap();
    source_list.remove_by_url(&url);
    source_list.save().unwrap_or_else(|error| {
        println!("Problem saving the sources: {:?}", error);
    });
}


#[tauri::command]
fn sync_source(state: tauri::State<LockedSourceList>, url: String) {
    let mut mutex_gd = state.0.lock().unwrap();
    let source_list = mutex_gd.as_mut().unwrap();
    source_list.sync(&url);
}

#[tauri::command]
fn sync_all_sources(state: tauri::State<LockedSourceList>) {
    let mut mutex_gd = state.0.lock().unwrap();
    let source_list = mutex_gd.as_mut().unwrap();
    source_list.sync_all();
}

#[tauri::command]
fn get_source_list_as_json(state: tauri::State<LockedSourceList>) -> String {
    let mutex_gd = state.0.lock().unwrap();
    let source_list = mutex_gd.as_ref().unwrap();
    serde_json::to_string(&source_list).expect("to json failed")
    // let mut html = String::new();
    // // html += "<div class=\"source\">";
    // // html += "<div class=\"name\">Name</div>";
    // // html += "<div class=\"timestamp\">Last Sync</div>";
    // // html += "<div class=\"update_rate\">Updates</div>";
    // // html += "<div class=\"status\">OK</div>";
    // // html += "</div>";
    // for source in source_list {
    //     html += "<div class=\"source\">";
    //     html += "<div class=\"info\">";
    //     html += &format!("<div class=\"name\">{}</div>", source.name());
    //     html += &format!("<div class=\"timestamp\">{}</div>", source.last_sync());
    //     html += &format!("<div class=\"update_rate\">{:.0} / day</div>", source.update_rate()*24.0);
    //     html += &format!("<div class=\"icon\">{}</div>", source.status());
    //     html += &format!("<div class=\"icon\" onclick=\"edit_source('{}')\">🖉</div>", source.url());
    //     html += &format!("<div class=\"icon\" onclick=\"remove_source('{}','{}')\">🗑</div>",source.name(), source.url());
    //     html += "</div>";
    //     // html += &format!("<div class=\"url\">{}</div>", source.url);
    //     html += "</div>";
    // }
    // html += "</div>";
    // html
}

#[tauri::command]
fn get_item_list_as_json(state: tauri::State<LockedSourceList>) -> String {
    let mutex_gd = state.0.lock().unwrap();
    let source_list = mutex_gd.as_ref().unwrap();
    let item_list = source_list.get_items();
    serde_json::to_string(&item_list).expect("to json failed")
    // let mut html = String::new();
    // for item in &item_list {
    //     html += "<div class=\"news_item\">";
    //     html += &format!("<div class=\"source_name\">{}</div>", item.source());
    //     html += &format!("<div class=\"timestamp\">{}</div>", item.timestamp());
    //     html += &format!("<div class=\"title\">{} <a href=\"{}\" target=\"_blank\">⬀</a></div>", item.title(), item.url());
    //     html += &format!("<div class=\"summary\">{}</div>", get_short_summary(&item.summary(), 100));
    //     html += "</div>";
    // }
    // html
}

#[tauri::command]
fn load_known_sources(handle: tauri::AppHandle) -> String {
   let resource_dir = handle.path_resolver()
      .resource_dir()
      .expect("failed to find resource dir");
    let known_sources_dir = resource_dir.join("known_sources");
    let paths = fs::read_dir(known_sources_dir).unwrap();
    let mut ret = "".to_string();
    for path in paths {
        if path.is_ok() {
            let path = path.unwrap().path();
            if path.is_file() {
                //println!("file {}", path.as_path().display());
                let mut opt_gp = "".to_string();
                let mut source_list = SourceList::new();
                if source_list.load(path).is_ok() {
                    opt_gp += &format!("<optgroup label=\"{}\">", source_list.name());
                    for source in &source_list {
                        opt_gp += &format!("<option value=\"{}\">{}</option>", source.url(), source.name());
                    }
                    opt_gp += "</optgroup>";
                    ret += &opt_gp;
                }
            }
        }
    }
    ret


}

struct LockedSourceList(Mutex<Option<SourceList>>);

fn main() {
    let user_sources = LockedSourceList(Mutex::new(Some(SourceList::new())));
    tauri::Builder::default()
        .manage(user_sources)
        .invoke_handler(tauri::generate_handler![
            load_user_sources, 
            load_known_sources,
            remove_source,
            add_source,
            sync_source, 
            sync_all_sources,
            get_source_list_as_json,
            get_item_list_as_json,
            get_short_summary,
            get_readable
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

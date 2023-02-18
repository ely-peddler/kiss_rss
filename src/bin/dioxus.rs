#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    //image::load_icon(Path::new("icon.png"));
    let style = include_str!("style.css").to_string();
    dioxus_desktop::launch_cfg(App,
        dioxus_desktop::Config::default()
            .with_custom_head(format!("<style>{style}</style>"))
            .with_window(dioxus_desktop::WindowBuilder::default()
                .with_title("Kiss RSS")
            )
        )
}

fn App(cx: Scope) -> Element {
    cx.render(
        rsx! {
            Sidebar(cx)
            div { class: "container", Items(cx) }
        }
    )
}

fn Sidebar(cx: Scope) -> Element {
    cx.render(
        rsx! {
            div { class: "sidebar",
                ul { class: "actions",
                    li { 
                    id: "reload", 
                    "⟳" } 
                }
                ul { class: "feeds",
                    li { id: "all", "∀" }
                }
                ul { class: "settings",  
                    li { id: "settings", "⛭" }
                }
            }
        }
    )
}

fn Items(cx: Scope) -> Element {
    let future = use_future(cx, (), |_| async move {
        kiss_rss::refresh_async()
            .await
    });
    cx.render(match future.value() {
        Some(Ok(items)) => rsx! { 
            div { id: "items", class: "overflowing", 
            items.iter().map(|i| rsx!{ 
                    div { 
                        class: "kiss_rss-news_item",
                        div { class: "kiss_rss-subscription", "{i.subscription}" }
                        div { class: "kiss_rss-timestamp", "{i.timestamp}" }
                        div { class: "kiss_rss-title", a { href: "{i.url}", "{i.title}" } }
                    }
                }
            ) }
        },
        Some(Err(_)) => rsx! { div { class: "kiss_rss-news_item", "Loading RSS failed" } },
        None => rsx! { div { class: "kiss_rss-news_item", "Loading RSS..." } }
    }
    )
}

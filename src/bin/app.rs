#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

extern crate dioxus_desktop;

fn main() {
    let style = include_str!("style.css").to_string();
    //style { [rsx!{include_str!("../src/style.css")}].into_iter() }
    dioxus_desktop::launch_cfg(App,
        dioxus_desktop::Config::default()
            .with_custom_head(format!("<style>{style}</style>"))
            .with_window(dioxus_desktop::WindowBuilder::default()
                .with_title("Kiss RSS")
                .with_decorations(true)
            )
        )
}

fn App(cx: Scope) -> Element {
    cx.render(
        rsx! {
            div { 
                // class: "frame",
                // div { class: "titlebar" }
                div { Items(cx) }
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
            div { 
                class: "kiss_rss-news_item",
                items.iter().map(|i| rsx!{ 
                    div { class: "kiss_rss-subscription", "{i.subscription}" }
                    div { class: "kiss_rss-timestamp", "{i.timestamp}" }
                    div { class: "kiss_rss-title", a { href: "{i.url}", "{i.title}" } }
                    }
                )
            }
        },
        Some(Err(_)) => rsx! { div { "Loading RSS failed" } },
        None => rsx! { div { "Loading RSS..." } }
    }
    )
}

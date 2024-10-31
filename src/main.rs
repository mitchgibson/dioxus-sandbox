#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
mod components; // Add this line

mod modules;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let mut text_input = use_signal(|| String::new());
    let on_input = move |evt: Event<FormData>| {
        text_input.set(evt.value().clone());
    };
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        input {
            r#type: "text",
            name: "example_input",
            value: text_input,
            oninput: on_input
        }
        p { class: "text-white", "Text: {text_input}"}
    }
}

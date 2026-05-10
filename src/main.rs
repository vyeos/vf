use std::collections::HashSet;

use dioxus::prelude::*;
mod components;
mod fs;
mod models;
use crate::{
    components::{FileList, Menubar, Sidebar},
    models::AppState,
};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| AppState {
        current_path: Signal::new(std::env::current_dir().unwrap()),
        favourites: Signal::new(vec![]),
        selected_items: Signal::new(HashSet::new()),
    });

    rsx! {
        document::Link {rel: "stylesheet", href: TAILWIND_CSS}
        div{
            class: "flex h-screen w-full",
            Sidebar{}
            div {
                class: "flex-1 flex flex-col",
                Menubar {  }
                FileList {  }
            }
        }
    }
}

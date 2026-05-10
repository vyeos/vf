use dioxus::prelude::*;
mod components;
mod fs;
mod models;
use crate::components::{FileList, Menubar, Sidebar};
// use fs::operations::read_dir;
// use std::env;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {rel: "stylesheet", href: TAILWIND_CSS}
        div{
            class: "flex",
            Sidebar{}
            div {
                class: "flex-1",
                Menubar {  }
                FileList {  }
            }
        }
    }
}

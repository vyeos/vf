use dioxus::prelude::*;
use dioxus_icons::lucide::{File, Folder};

use crate::{fs::operations::read_dir, models::AppState};

#[component]
pub fn FileList() -> Element {
    let mut app_state = use_context::<AppState>();
    let files = use_memo(move || {
        let path = (app_state.current_path)();
        read_dir(&path)
    });

    rsx! {
        div{
            class: "flex-1 border-l border-2 border-t p-4 flex flex-col justify-top gap-2 flex-wrap overflow-auto",
            for item in files() {
                div {
                    onclick: move |_| {
                        if item.is_dir {
                            app_state.current_path.set(item.path.clone());
                        }
                    },
                    key: "{item.path.display()}",
                    {if item.is_dir {
                        rsx! {div{
                            class: "flex gap-2 items-center cursor-pointer text-primary hover:bg-secondary/20",
                            Folder{size: 16}
                           "{item.name}"
                        }}
                    } else {
                        rsx!{div {
                            class: "flex gap-2 items-center hover:bg-secondary/20",
                            File{size: 16}
                           "{item.name}"
                        }}
                    }}
                }
            }
        }
    }
}

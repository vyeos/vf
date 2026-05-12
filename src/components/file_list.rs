use dioxus::prelude::*;
use dioxus_icons::lucide::{File, Folder};

use crate::{
    fs::operations::read_dir,
    models::{AppState, FileItem},
};

#[component]
pub fn FileList() -> Element {
    let mut app_state = use_context::<AppState>();
    let files = use_memo(move || {
        let path = (app_state.current_path)();
        read_dir(&path)
    });

    rsx! {
        div {
            class: "flex-1 border-l border-2 border-t p-4 flex flex-col justify-top gap-1 overflow-auto",
            onclick: move |_| app_state.clear_selection(),
            for item in files() {
                FileRow { key: "{item.path.display()}", item }
            }
        }
    }
}

#[component]
fn FileRow(item: FileItem) -> Element {
    let app_state = use_context::<AppState>();
    let is_selected = app_state.is_selected(&item.path);
    let row_class = if is_selected {
        "flex gap-2 items-center rounded-md px-2 py-1 cursor-pointer bg-primary text-primary-foreground"
    } else {
        "flex gap-2 items-center rounded-md px-2 py-1 cursor-pointer hover:bg-secondary/20"
    };
    let select_path = item.path.clone();
    let mut select_state = app_state.clone();
    let mut open_state = app_state.clone();
    let open_item = item.clone();

    rsx! {
        div {
            class: "{row_class}",
            onclick: move |event| {
                event.stop_propagation();
                let keep_existing = event.modifiers().ctrl() || event.modifiers().meta();
                select_state.select_item(select_path.clone(), keep_existing);
            },
            ondoubleclick: move |event| {
                event.stop_propagation();
                open_state.open_item(&open_item);
            },
            if item.is_dir {
                Folder { size: 16 }
            } else {
                File { size: 16 }
            }
            span {
                class: "truncate",
                "{item.name}"
            }
        }
    }
}

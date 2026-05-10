use dioxus::prelude::*;

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
                            svg{ xmlns:"http://www.w3.org/2000/svg",
                                width:"16",
                                height:"16",
                                view_box:"0 0 24 24",
                                fill:"none",
                                stroke:"currentColor",
                                stroke_width:"2",
                                stroke_linecap:"round",
                                stroke_linejoin:"round",
                                class:"lucide lucide-folder-icon lucide-folder",
                                path {d:"M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"}
                            }
                           "{item.name}"
                        }}
                    } else {
                        rsx!{div {
                            class: "flex gap-2 items-center hover:bg-secondary/20",
                            svg {
                                xmlns:"http://www.w3.org/2000/svg",
                                width:"16",
                                height:"16",
                                view_box:"0 0 24 24",
                                fill:"none",
                                stroke:"currentColor",
                                stroke_width:"2",
                                stroke_linecap:"round",
                                stroke_linejoin:"round",
                                class:"lucide lucide-file-icon lucide-file",
                                path {d:"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"}
                                path {d:"M14 2v5a1 1 0 0 0 1 1h5"}
                            }
                           "{item.name}"
                        }}
                    }}
                }
            }
        }
    }
}

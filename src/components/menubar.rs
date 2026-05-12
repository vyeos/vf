use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronLeft, ChevronRight, Minus, Plus};

use crate::models::AppState;

#[component]
pub fn Menubar() -> Element {
    let mut app_state = use_context::<AppState>();
    let curr = (app_state.current_path)();

    rsx! {
        div{
            class: "h-10 w-full p-2 flex items-center justify-between",
            div {
                class: "flex items-center justify-center gap-2 p-1 text-secondary-foreground bg-secondary rounded-full",
                button {
                    onclick: move |_| {
                        if let Some(parent) = curr.parent() {
                            app_state.set_current_path(parent.to_path_buf());
                        }
                    },
                    class: " hover:bg-primary hover:text-primary-foreground rounded-full hover:cursor-pointer",
                    ChevronLeft{}
                }
                div {
                   class: "-mx-2 rotate-90",
                    Minus {}
                }
                button{
                    class: " hover:bg-primary hover:text-primary-foreground rounded-full hover:cursor-pointer",
                    ChevronRight{}
                }
            }
            div {
                class: "flex items-center justify-center p-1 bg-secondary text-secondary-foreground rounded-full hover:cursor-pointer",
                onclick: move |_| {},
                button {
                    Plus{}
                }
            }
        }
    }
}

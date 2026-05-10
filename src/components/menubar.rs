use dioxus::prelude::*;

#[component]
pub fn Menubar() -> Element {
    rsx! {
        div{class: "h-20 w-full text-primary",
            h1 {
                "hi! Menubar here"
            }
        }
    }
}

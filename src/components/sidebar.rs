use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div{class: "h-screen min-w-45 max-w-70 px-4 py-2 text-secondary",
            "hi! "
            "sidebar here"
        }
    }
}

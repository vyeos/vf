use dioxus::prelude::*;

use crate::models::AppState;

#[component]
pub fn Menubar() -> Element {
    let mut app_state = use_context::<AppState>();
    let curr= (app_state.current_path)();
        
    rsx! {
        div{
            class: "h-10 w-full p-2 flex items-center",
            div {
                class: "flex items-center justify-center gap-2 p-1 text-secondary-foreground bg-secondary rounded-full",
                svg {
                    onclick: move |_| {
                        if let Some(parent) = curr.parent() {
                            app_state.current_path.set(parent.to_path_buf());
                        }
                    },
                    class:"lucide lucide-chevron-left-icon lucide-chevron-left hover:bg-primary hover:text-primary-foreground rounded-full hover:cursor-pointer",
                    xmlns:"http://www.w3.org/2000/svg",
                    width:"24",
                    height:"24",
                    view_box:"0 0 24 24",
                    fill:"none",
                    stroke:"currentColor",
                    stroke_width:"2",
                    stroke_linecap:"round",
                    stroke_linejoin:"round",
                    path{ d:"m15 18-6-6 6-6"}
                }
                svg{
                    xmlns:"http://www.w3.org/2000/svg",
                    width:"24",
                    height:"24",
                    view_box:"0 0 24 24",
                    fill:"none",
                    stroke:"currentColor",
                    stroke_width:"2",
                    stroke_linecap:"round",
                    stroke_linejoin:"round",
                    class:"lucide lucide-minus-icon lucide-minus rotate-90 -mx-2",
                    path{ d:"M5 12h14"}
                }
                svg{
                    xmlns:"http://www.w3.org/2000/svg",
                    width:"24",
                    height:"24",
                    view_box:"0 0 24 24",
                    fill:"none",
                    stroke:"currentColor",
                    stroke_width:"2",
                    stroke_linecap:"round",
                    stroke_linejoin:"round",
                    class:"lucide lucide-chevron-right-icon lucide-chevron-right hover:bg-primary hover:text-primary-foreground rounded-full hover:cursor-pointer",
                    path {d:"m9 18 6-6-6-6"}
                }
            }
        }
    }
}

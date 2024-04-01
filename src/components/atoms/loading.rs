use dioxus::prelude::*;

#[component]
pub fn loading() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center",
            {
                rsx! {
                    div {
                        class: "animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-gray-900",
                    }
                }
            }
        }
    }
}

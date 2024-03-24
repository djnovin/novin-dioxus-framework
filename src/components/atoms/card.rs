use dioxus::prelude::*;

#[component]
pub fn card(children: Vec<Element>) -> Element {
    rsx! {
        div {
            class: "p-6 bg-white rounded-sm border border-gray-200 shadow-sm flex flex-col",
                {children}.iter()
        }
    }
}

use dioxus::prelude::*;

#[component]
pub fn skeleton(class: String) -> Element {
    let base_class = "animate-pulse bg-gray-200";
    let skeleton_class = format!("{}", base_class);

    rsx! {
        div {
            class: format!("{} {}", skeleton_class, class)
        }
    }
}

use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum SeperatorOrientation {
    Horizontal,
    Vertical,
}

#[component]
pub fn seperator(class: String, orientation: SeperatorOrientation) -> Element {
    let base_class = "border-t border-gray-200";
    let seperator_class = match orientation {
        SeperatorOrientation::Horizontal => format!("{} w-full", base_class),
        SeperatorOrientation::Vertical => format!("{} h-full", base_class),
    };
    rsx! {
        hr {
            class: seperator_class
        }
    }
}

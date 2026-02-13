use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum SeperatorOrientation {
    Horizontal,
    Vertical,
}

#[derive(Props, PartialEq, Clone)]
pub struct SeperatorProps {
    pub class: String,
    pub orientation: SeperatorOrientation,
}

#[component]
pub fn Seperator(props: SeperatorProps) -> Element {
    let base_class = "border-t border-gray-200";
    let seperator_class = match props.orientation {
        SeperatorOrientation::Horizontal => format!("{} w-full", base_class),
        SeperatorOrientation::Vertical => format!("{} h-full", base_class),
    };
    rsx! {
        hr { class: seperator_class }
    }
}

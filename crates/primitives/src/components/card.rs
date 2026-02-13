use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct CardProps {
    pub children: Vec<Element>,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let children = props.children;
    rsx! {
        div { class: "p-6 bg-white rounded-sm border border-gray-200 shadow-sm flex flex-col",
            { children }.iter()
        }
    }
}

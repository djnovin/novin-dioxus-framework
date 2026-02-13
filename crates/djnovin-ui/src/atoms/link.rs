use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
struct LinkProps {
    href: String,
    children: Element,
}

#[component]
pub fn Link(props: LinkProps) -> Element {
    let base_class = "text-blue-500";

    rsx! {
        a { class: base_class, href: props.href, {props.children} }
    }
}

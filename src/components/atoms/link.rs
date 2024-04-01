use dioxus::prelude::*;

struct LinkProps {
    href: String,
    children: Element,
}

#[component]
fn link(props: LinkProps) -> Element {
    let base_class = "text-blue-500";

    rsx! {
        a {
            class: base_class,
            href: props.href,
            {props.children}
        }
    }
}

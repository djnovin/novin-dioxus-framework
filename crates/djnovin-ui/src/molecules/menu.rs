use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MenuItems {
    pub name: String,
    pub link: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MenuProps {
    pub items: Vec<MenuItems>,
}

#[component]
pub fn menu(props: MenuProps) -> Element {
    rsx! {
        div {
            class: "flex flex-row gap-4",
            {props.items.iter().map(|item| {
                rsx! {
                    a {
                        href: item.link,
                        {item.name}
                    }
                }
            })}
        }
    }
}

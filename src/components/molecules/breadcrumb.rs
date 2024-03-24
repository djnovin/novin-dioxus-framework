use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreadcrumbItemProps {
    pub items: Vec<BreadcrumbItem>,
}

#[component]
pub fn breadcrumb(props: BreadcrumbItemProps) -> Element {
    let ellipsis = BreadcrumbItem {
        label: "...".to_string(),
        href: "".to_string(),
    };

    if props.items.len() >= 4 {
        let mut new_items = vec![];
        new_items.push(props.items[0].clone());
        new_items.push(ellipsis);
        for i in 2..props.items.len() {
            new_items.push(props.items[i].clone());
        }
        return rsx! {
            div {
                class: "flex items-center space-x-2",
                {
                new_items.iter().map(|item| {
                    rsx! {
                        a {
                            class: {
                                if item != props.items.last().unwrap() {
                                    "text-gray-500"
                                } else {
                                    "text-black"
                                }
                            },
                            href: item.href.clone(),
                            {item.label.clone()}
                        },
                        if item != &props.items[props.items.len() - 1] {
                            span {">"}
                        } else {
                            span {""}
                        }
                    }
                })
                }
            }
        };
    }
    rsx! {
        div {
            class: "flex items-center space-x-2",
            {
            props.items.iter().map(|item| {
                rsx! {
                    a {
                        class: "hover:underline",
                        href: item.href.clone(),
                        {item.label.clone()}
                    },
                    if item != &props.items[props.items.len() - 1] {
                        span {">"}
                    } else {
                        span {""}
                    }
                }
            })
            }
        }
    }
}

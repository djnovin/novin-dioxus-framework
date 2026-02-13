use dioxus::prelude::*;

use crate::{
    typography::{Typography, TypographyProps, TypographyVariant},
};

#[derive(Props, Clone, PartialEq)]
pub struct AccordianProps {
    pub title: String,
    pub content: String,
    pub is_open: Option<bool>,
    pub on_toggle: Option<EventHandler>,
    pub class: Option<String>,
}

#[component]
pub fn Accordian(props: AccordianProps) -> Element {
    let mut is_open = use_signal(|| props.is_open.unwrap_or(false));

    rsx! {
        div {
            onclick: move |_| {
                is_open.set(!is_open());
                if let Some(on_toggle) = &props.on_toggle {
                    on_toggle.call(());
                }
            },
            class: "flex flex-col gap-4 py-4 w-full group",
            div { class: "flex flex-row justify-between items-center space-x-4 cursor-pointer w-full",
                div {
                    {
                        Typography(TypographyProps {
                            variant: TypographyVariant::H6,
                            class: Some("font-light group-hover:underline".to_string()),
                            children: rsx! { "{props.title}" },
                        })
                    }
                }
                div { class: "text-primary",
                    if is_open() == true {
                        "-"
                    } else {
                        "+"
                    }
                }
            }
            div {
                if is_open() {
                    div { class: "flex flex-col gap-4",
                        div {
                            {
                                Typography(TypographyProps {
                                    class: Some("font-light".to_string()),
                                    children: rsx! { "{props.content}" },
                                    variant: TypographyVariant::Body2,
                                })
                            }
                        }
                    }
                } else {
                    div { class: "hidden" }
                }
            }
        }
    }
}

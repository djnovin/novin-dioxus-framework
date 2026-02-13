use dioxus::prelude::*;

use crate::components::{self, atoms::typography::typographyProps};

#[component]
pub fn accordian() -> Element {
    let mut is_open = use_signal(|| false);

    rsx! {
    div {
        onclick: move |_| {
            if is_open() == true {
                is_open.set(false);
                return;
            } else {
                is_open.set(true);
                return;
            }
        },
        class: "flex flex-col gap-4 py-4 w-full group",
        div {
            class: "flex flex-row justify-between items-center space-x-4 cursor-pointer w-full",
            div {
                {
                    components::atoms::typography::typography(typographyProps {
                        class: Some("font-light group-hover:underline".to_string()),
                        children: rsx! {
                            "Accordian Title"
                        },
                        props: components::atoms::typography::TypographyProps {
                            variant: components::atoms::typography::TypographyVariant::H6
                        }
                    })
                }
            }
            div {
                class: "text-primary",
                if is_open() == true {
                    "-"
                } else {
                    "+"
                }
            }
        }
        div {
            if is_open() {
                div {
                    class: "flex flex-col gap-4",
                    div {
                        {
                            components::atoms::typography::typography(typographyProps {
                                class: Some("font-light".to_string()),
                                children: rsx! {
                                    "A complete component library for Rust, built using Dioxus and TailwindCSS. Beautiful components that can simply be copy-pasted into your project. Accessible. Customizable. Open Source."
                                },
                                props: components::atoms::typography::TypographyProps {
                                    variant: components::atoms::typography::TypographyVariant::Body2
                                }
                            })
                        }
                    }
                }
            } else {
                div {
                    class: "hidden"
                }
            }
        }
    }
    }
}

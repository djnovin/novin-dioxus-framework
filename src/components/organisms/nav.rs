use dioxus::prelude::*;

use crate::components::{
    self,
    atoms::{button::buttonProps, typography::typographyProps},
};

#[component]
pub fn nav() -> Element {
    rsx! {
        div {
        class: "flex justify-between flex-row items-center p-4 fixed top-0 left-0 right-0 z-50 w-full border-b border-border/40 bg-white backdrop-blur-md supports-[backdrop-filter]:bg-black/60",
            {
                components::atoms::typography::typography(typographyProps {
                    class: Some("font-bold flex flex-row gap-8".to_string()),
                    children: rsx! {
                        div {
                            "@djnov/cnrs"
                        }
                        div {
                            "Component Library"
                        }
                    },
                    props: components::atoms::typography::TypographyProps {
                        variant: components::atoms::typography::TypographyVariant::H4
                    }
                })
            }
            {
                components::atoms::button::button(buttonProps {
                    children: rsx! { "Get Started" },
                    button_type: components::atoms::button::ButtonType::Primary,
                    button_size: components::atoms::button::ButtonSize::Default
                })
            }
        }
    }
}

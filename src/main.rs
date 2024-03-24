#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::{
    atoms::{
        avatar::avatarProps, badge::badgeProps, button::buttonProps, card::cardProps,
        input::inputProps, seperator::seperatorProps, skeleton::skeletonProps, toggle::toggleProps,
        typography::typographyProps,
    },
    molecules::{
        alert::{alertProps, AlertVariant},
        breadcrumb::breadcrumbProps,
        tooltip::tooltipProps,
    },
    organisms::section::sectionProps,
};

mod components {
    pub mod molecules {
        pub mod accordian;
        pub mod alert;
        pub mod breadcrumb;
        pub mod tooltip;
    }
    pub mod atoms {
        pub mod avatar;
        pub mod badge;
        pub mod button;
        pub mod card;
        pub mod input;
        pub mod seperator;
        pub mod skeleton;
        pub mod toggle;
        pub mod typography;
    }
    pub mod organisms {
        pub mod nav;
        pub mod section;
    }
}

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx! {
        main {
            {
                components::organisms::nav::nav()
            }
            {
                components::organisms::section::section(sectionProps {
                    props: components::organisms::section::SectionProps {
                        class: Some("px-4".to_string()).into(),
                        Type: components::organisms::section::SectionType::Full
                    },
                    children: {
                        rsx! {
                            div {
                                class: "flex justify-center flex-col space-y-8 items-center h-full",
                                {
                                    components::atoms::badge::badge(badgeProps {
                                        children: rsx! {
                                            {
                                                components::atoms::typography::typography(typographyProps {
                                                    class: Some("text-center".to_string()),
                                                    children: rsx! { "New Components Coming Soon" },
                                                    props: components::atoms::typography::TypographyProps {
                                                        variant: components::atoms::typography::TypographyVariant::Overline
                                                    }
                                                })
                                            }
                                        },
                                        variant: components::atoms::badge::BadgeVariants::Secondary
                                    })
                                }
                                {
                                    components::atoms::typography::typography(typographyProps {
                                        class: Some("text-center font-bold".to_string()),
                                        children: rsx! { "Component Library for Rust" },
                                        props: components::atoms::typography::TypographyProps {
                                            variant: components::atoms::typography::TypographyVariant::H1
                                        }
                                    })
                                }
                                {
                                    components::atoms::typography::typography(typographyProps {
                                        class: Some("text-center".to_string()),
                                        children: rsx! { "A complete component library for Rust, built using Dioxus and TailwindCSS. Beautiful components that can simply be copy-pasted into your project. Accessible. Customizable. Open Source." },
                                        props: components::atoms::typography::TypographyProps {
                                            variant: components::atoms::typography::TypographyVariant::H2
                                        }
                                    })
                                }
                                div {
                                    class: "flex space-x-2",
                                    {
                                        components::atoms::button::button(buttonProps {
                                            children: rsx! { "Get Started" },
                                            button_type: components::atoms::button::ButtonType::Primary,
                                            button_size: components::atoms::button::ButtonSize::Default
                                        })
                                    }
                                    {
                                        components::atoms::button::button(buttonProps {
                                            children: rsx! { "GitHub" },
                                            button_type: components::atoms::button::ButtonType::Secondary,
                                            button_size: components::atoms::button::ButtonSize::Default
                                        })
                                    }
                                }
                            }
                        }
                    }
                })
            }
            div {
                class: "flex space-y-4 flex-col",
                {
                    components::molecules::accordian::accordian()
                }
            }
            div {
                class: "absolute bottom-0 right-0 left-0 p-4",
                {
                    components::molecules::alert::alert(alertProps {
                        variant: AlertVariant::Info,
                        children: rsx! {
                            {
                                components::atoms::typography::typography(typographyProps {
                                    class: None,
                                    children: rsx! { "Component Library is currently in pre-release development. Use with caution." },
                                    props: components::atoms::typography::TypographyProps {
                                        variant: components::atoms::typography::TypographyVariant::Body1
                                    }
                                })
                            }
                        }
                    })
                }
            }
            div {
                class: "flex space-x-2",
                {
                    components::atoms::badge::badge(badgeProps {
                        children: rsx! { "Badge" },
                        variant: components::atoms::badge::BadgeVariants::Primary
                    })
                }
                {
                    components::atoms::badge::badge(badgeProps {
                        children: rsx! { "Badge" },
                        variant: components::atoms::badge::BadgeVariants::Secondary
                    })
                }
                {
                    components::atoms::badge::badge(badgeProps {
                        children: rsx! { "Badge" },
                        variant: components::atoms::badge::BadgeVariants::Outline
                    })
                }
                {
                    components::atoms::badge::badge(badgeProps {
                        children: rsx! { "Badge" },
                        variant: components::atoms::badge::BadgeVariants::Destructive
                    })
                }
            }
            {
                components::atoms::seperator::seperator(seperatorProps {
                    orientation: components::atoms::seperator::SeperatorOrientation::Horizontal,
                    class: "my-8".to_string()
                })
            }
            div {
                {
                    components::molecules::breadcrumb::breadcrumb(breadcrumbProps {
                        props: components::molecules::breadcrumb::BreadcrumbItemProps {
                            items: vec![
                                components::molecules::breadcrumb::BreadcrumbItem {
                                    label: "Home".to_string(),
                                    href: "/".to_string(),
                                },
                                components::molecules::breadcrumb::BreadcrumbItem {
                                    label: "About".to_string(),
                                    href: "/about".to_string(),
                                },
                                components::molecules::breadcrumb::BreadcrumbItem {
                                    label: "Contact".to_string(),
                                    href: "/contact".to_string(),
                                },
                                components::molecules::breadcrumb::BreadcrumbItem {
                                    label: "Blog".to_string(),
                                    href: "/blog".to_string(),
                                },
                            ]
                        }
                    })
                }
            }
            div {
                class: "flex space-y-2 flex-col",
                {
                    components::atoms::typography::typography(typographyProps {
                        class: None,
                        children: rsx! { "Heading 1" },
                        props: components::atoms::typography::TypographyProps {
                            variant: components::atoms::typography::TypographyVariant::H1
                        }
                    })
                }
                {
                    components::atoms::typography::typography(typographyProps {
                        class: None,
                        children: rsx! { "Heading 2" },
                        props: components::atoms::typography::TypographyProps {
                            variant: components::atoms::typography::TypographyVariant::H2
                        }
                    })
                }
                {
                    components::atoms::typography::typography(typographyProps {
                        class: None,
                        children: rsx! { "Heading 3" },
                        props: components::atoms::typography::TypographyProps {
                            variant: components::atoms::typography::TypographyVariant::H3
                        }
                    })
                }
                {
                    components::atoms::typography::typography(typographyProps {
                        class: None,
                        children: rsx! { "Heading 4" },
                        props: components::atoms::typography::TypographyProps {
                            variant: components::atoms::typography::TypographyVariant::H4
                        }
                    })
                }
                {
                    components::atoms::typography::typography(typographyProps {
                        class: None,
                        children: rsx! { "Heading 5" },
                        props: components::atoms::typography::TypographyProps {
                            variant: components::atoms::typography::TypographyVariant::H5
                        }
                    })
                }
                {
                    components::atoms::typography::typography(typographyProps {
                        class: None,
                        children: rsx! { "Heading 6" },
                        props: components::atoms::typography::TypographyProps {
                            variant: components::atoms::typography::TypographyVariant::H6
                        }
                    })
                }
                {
                    components::atoms::typography::typography(typographyProps {
                        class: None,
                        children: rsx! { "Subtitle 1" },
                        props: components::atoms::typography::TypographyProps {
                            variant: components::atoms::typography::TypographyVariant::Subtitle1
                        }
                    })
                }
                {
                    components::atoms::typography::typography(typographyProps {
                        class: None,
                        children: rsx! { "Subtitle 2" },
                        props: components::atoms::typography::TypographyProps {
                            variant: components::atoms::typography::TypographyVariant::Subtitle2
                        }
                    })
                }
                {
                    components::atoms::typography::typography(typographyProps {
                        class: None,
                        children: rsx! { "Body 1" },
                        props: components::atoms::typography::TypographyProps {
                            variant: components::atoms::typography::TypographyVariant::Body1
                        }
                    })
                }
                {
                    components::atoms::typography::typography(typographyProps {
                        class: None,
                        children: rsx! { "Body 2" },
                        props: components::atoms::typography::TypographyProps {
                            variant: components::atoms::typography::TypographyVariant::Body2
                        }
                    })
                }
                {
                    components::atoms::typography::typography(typographyProps {
                        class: None,
                        children: rsx! { "Caption" },
                            props: components::atoms::typography::TypographyProps {
                                variant: components::atoms::typography::TypographyVariant::Caption
                            }
                        })
                    }
                    {
                        components::atoms::typography::typography(typographyProps {
                            class: None,
                            children: rsx! { "Overline" },
                            props: components::atoms::typography::TypographyProps {
                                variant: components::atoms::typography::TypographyVariant::Overline
                            }
                        })
                    }
                    {
                        components::atoms::typography::typography(typographyProps {
                            class: None,
                            children: rsx! { "Blockquote" },
                            props: components::atoms::typography::TypographyProps {
                                variant: components::atoms::typography::TypographyVariant::Blockquote
                            }
                        })
                    }
                    {
                        components::atoms::typography::typography(typographyProps {
                            class: None,
                            children: rsx! { "Muted" },
                            props: components::atoms::typography::TypographyProps {
                                variant: components::atoms::typography::TypographyVariant::Muted
                            }
                        })
                    }
                }
                div {
                    class: "flex space-x-2",
                    {
                        components::atoms::toggle::toggle(toggleProps {
                            props: components::atoms::toggle::ToggleProps {
                                border: false,
                                size: components::atoms::toggle::ToggleSize::Small,
                                disabled: false
                            },
                            children: rsx! {"B"}
                        })
                    }
                }
                div {
                    class: "flex space-x-2",
                    {
                        components::molecules::tooltip::tooltip(tooltipProps {
                            props: components::molecules::tooltip::TooltipProps {
                                content: "Tooltip".to_string(),
                                position: components::molecules::tooltip::TooltipPosition::Top
                            },
                            children: rsx! {
                                {
                                    components::atoms::button::button(buttonProps {
                                        children: rsx! {"Hover me"},
                                        button_type: components::atoms::button::ButtonType::Primary,
                                        button_size: components::atoms::button::ButtonSize::Default
                                    })
                                }
                            }
                        })
                    }
                }
                div {
                    class: "flex space-x-2 flex-row items-center",
                    {
                        components::atoms::avatar::avatar(avatarProps {size:components::atoms::avatar::AvatarSize::Small, children: rsx! {"👋"}})
                    }
                    {
                        components::atoms::avatar::avatar(avatarProps {size:components::atoms::avatar::AvatarSize::Medium, children: rsx! {"👋"}})
                    }
                    {
                        components::atoms::avatar::avatar(avatarProps {size:components::atoms::avatar::AvatarSize::Large, children: rsx! {"👋"}})
                    }
                }
                div {
                    {
                        components::atoms::skeleton::skeleton(skeletonProps {
                            class: "h-[125px] w-[250px] rounded-xl".to_string()
                        })
                    }
                }
                {
                components::atoms::card::card(cardProps {
                    children: vec![
                        rsx! {
                            div {
                                {
                                    components::atoms::input::input(inputProps {
                                        props: components::atoms::input::InputProps {
                                            placeholder: "Enter your name".to_string(),
                                            value: "John Doe".to_string()
                                        }
                                    })
                                }
                                {
                                components::atoms::button::button(buttonProps {
                                    children: rsx! {"Click me"},
                                    button_type: components::atoms::button::ButtonType::Primary,
                                    button_size: components::atoms::button::ButtonSize::Default
                            })}
                        }}
                    ]
                })
            }
        }
    }
}

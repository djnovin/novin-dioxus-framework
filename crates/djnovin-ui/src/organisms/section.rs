use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SectionType {
    Full,
    Grid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SectionProps {
    pub Type: SectionType,
    pub class: Option<String>,
}

#[component]
pub fn section(props: SectionProps, children: Element) -> Element {
    let class = format!("{}", props.class.unwrap_or_default());
    if props.Type == SectionType::Full {
        rsx! {
            section {
                class: format!("w-full h-screen {}", class),
                {children}
            }
        }
    } else if props.Type == SectionType::Grid {
        rsx! {
            section {
                class: format!("grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 px-4 {}", class),
                {children}
            }
        }
    } else {
        rsx! {
            section {
                {children}
            }
        }
    }
}

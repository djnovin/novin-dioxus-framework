use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum TileKind {
    Selection,
    Action,
}

#[derive(PartialEq, Clone)]
pub enum TileAlignment {
    Left,
    Center,
    Right,
}

#[derive(Clone, PartialEq)]
struct TileProps {
    label: String,
    kind: TileKind,
    leadingContent: Element,
    trailingContent: Element,
    headerAlignment: TileAlignment,
    bodyAlignment: TileAlignment,
    children: Element,
    selected: bool,
}

#[component]
pub fn tile(props: TileProps) -> Element {
    let base_class = "relative";

    let kind_class = match props.kind {
        TileKind::Selection => "bg-gray-50",
        TileKind::Action => "bg-white",
    };

    let header_alignment_class = match props.headerAlignment {
        TileAlignment::Left => "justify-start",
        TileAlignment::Center => "justify-center",
        TileAlignment::Right => "justify-end",
    };

    let body_alignment_class = match props.bodyAlignment {
        TileAlignment::Left => "justify-start",
        TileAlignment::Center => "justify-center",
        TileAlignment::Right => "justify-end",
    };

    rsx! {
        div {
            class: format!("{} {}", base_class, kind_class),
            {
                rsx! {
                    div {
                        class: "flex items-center",
                        {
                            rsx! {
                                div {
                                    class: format!("flex items-center space-x-2 p-2 {}", header_alignment_class),
                                    {props.leadingContent}
                                    {props.label}
                                    {props.trailingContent}
                                }
                            }
                        }
                    }
                    div {
                        class: format!("flex items-center space-x-2 p-2 {}", body_alignment_class),
                        {props.children}
                    }
                }
            }
        }
    }
}

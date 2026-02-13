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

#[derive(Props, Clone, PartialEq)]
pub struct TileProps {
    label: String,
    kind: TileKind,
    leading_content: Element,
    trailing_content: Element,
    header_alignment: TileAlignment,
    body_alignment: TileAlignment,
    children: Element,
    selected: bool,
}

#[component]
pub fn Tile(props: TileProps) -> Element {
    let base_class = "relative";

    let kind_class = match props.kind {
        TileKind::Selection => "bg-gray-50",
        TileKind::Action => "bg-white",
    };

    let header_alignment_class = match props.header_alignment {
        TileAlignment::Left => "justify-start",
        TileAlignment::Center => "justify-center",
        TileAlignment::Right => "justify-end",
    };

    let body_alignment_class = match props.body_alignment {
        TileAlignment::Left => "justify-start",
        TileAlignment::Center => "justify-center",
        TileAlignment::Right => "justify-end",
    };

    rsx! {
        div { class: format!("{} {}", base_class, kind_class),
            {
                rsx! {
                    div { class: "flex items-center",
                        {
                            rsx! {
                                div { class: format!("flex items-center space-x-2 rtl:space-x-reverse p-2 {}", header_alignment_class),
                                    {props.leading_content}
                                    {props.label}
                                    {props.trailing_content}
                                }
                            }
                        }
                    }
                    div { class: format!("flex items-center space-x-2 rtl:space-x-reverse p-2 {}", body_alignment_class), {props.children} }
                }
            }
        }
    }
}

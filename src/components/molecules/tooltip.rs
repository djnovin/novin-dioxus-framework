use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TooltipProps {
    pub content: String,
    pub position: TooltipPosition,
}

#[component]
pub fn tooltip(props: TooltipProps, children: Element) -> Element {
    let mut is_open = use_signal(|| false);

    let base_class = "relative inline-block";

    let tooltip_class = if is_open() {
        "opacity-100"
    } else {
        "opacity-0"
    };

    let tooltip_position = match props.position {
        TooltipPosition::Top => "top-0 left-1/2 transform -translate-x-1/2 -translate-y-full",
        TooltipPosition::Bottom => "bottom-0 left-1/2 transform -translate-x-1/2 translate-y-full",
        TooltipPosition::Left => "left-0 top-1/2 transform -translate-x-full -translate-y-1/2",
        TooltipPosition::Right => "right-0 top-1/2 transform translate-x-full -translate-y-1/2",
    };

    rsx! {
        div {
            class: base_class,
            onmouseenter: move |_| {
                is_open.set(true);
            },
            onmouseleave: move |_| {
                is_open.set(false);
            },
            {children}
            div {
                class: format!("absolute z-10 bg-gray-800 text-white text-xs px-2 py-1 rounded-md transition-all duration-150 delay-700 ease-linear {} {}", tooltip_position ,tooltip_class),
                {props.content}
            }
        }
    }
}

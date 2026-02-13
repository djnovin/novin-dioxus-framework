use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum ToggleSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, PartialEq)]
pub struct ToggleProps {
    pub border: bool,
    pub disabled: bool,
    pub size: ToggleSize,
}

#[component]
pub fn toggle(props: ToggleProps, children: Element) -> Element {
    let mut is_toggle = use_signal(|| false);

    let base_class = "relative rounded-sm transition duration-200 ease-linear aspect-square";

    let size_class = match props.size {
        ToggleSize::Small => "w-8 h-8",
        ToggleSize::Medium => "w-12 h-12",
        ToggleSize::Large => "w-16 h-16",
    };

    let border_class = if props.border { "border" } else { "" };

    let disabled_class = if props.disabled { "disabled" } else { "" };

    let toggle_class = if is_toggle() {
        "bg-gray-200"
    } else {
        "bg-gray-50"
    };

    rsx! {
        button {
            class: format!("{} {} {} {} {}", base_class, size_class, border_class, toggle_class, disabled_class),
            onclick: move |_| {
                is_toggle.set(!is_toggle().clone());
            },
            {children}
        }
    }
}

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Primary,
    Secondary,
    Outline,
    Link,
    Ghost,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Default,
    Small,
    Large,
    Icon,
}

#[component]
pub fn button(children: Element, button_type: ButtonType, button_size: ButtonSize) -> Element {
    let base_class = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

    let button_size_class = match button_size {
        ButtonSize::Default => "h-9 px-4 py-2",
        ButtonSize::Small => "h-8 rounded-md px-3 text-xs",
        ButtonSize::Large => "h-10 rounded-md px-8",
        ButtonSize::Icon => "h-9 w-9",
    };

    let button_variant_class = match button_type {
        ButtonType::Primary => "bg-black text-white",
        ButtonType::Secondary => "bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80",
        ButtonType::Outline => "border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
        ButtonType::Link => "text-primary underline-offset-4 hover:underline",
        ButtonType::Ghost => "hover:bg-accent hover:text-accent-foreground",
    };

    let button_class = format!(
        "{} {} {}",
        base_class, button_size_class, button_variant_class
    );

    rsx! {
        button {
            class: button_class,
            {children}
        }
    }
}

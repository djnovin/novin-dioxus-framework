use dioxus::prelude::*;
use primitives_core::button::BASE_CLASSES;

// Re-export core types so consumers don't need to depend on primitives-core
// directly.
pub use primitives_core::button::{ButtonSize, ButtonVariant};

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Visual style of the button.
    #[props(default)]
    pub variant: ButtonVariant,

    /// Controls height, padding, and icon sizing.
    #[props(default)]
    pub size: ButtonSize,

    /// Additional Tailwind classes merged onto the root element.
    #[props(default)]
    pub class: String,

    /// Maps to the HTML `disabled` attribute.
    #[props(default)]
    pub disabled: bool,

    /// Accessible label — required for icon-only buttons.
    #[props(default)]
    pub aria_label: Option<String>,

    /// HTML button type attribute (`"button"`, `"submit"`, `"reset"`).
    /// Defaults to `"button"`.
    #[props(default = "button".to_string())]
    pub r#type: String,

    /// Click handler.
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// Button content (text, icons, spinners, etc.).
    pub children: Element,
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A button component matching the shadcn/ui v4 API.
///
/// ## Variants
///
/// `Default` · `Destructive` · `Outline` · `Secondary` · `Ghost` · `Link`
///
/// ## Sizes
///
/// `Xs` · `Sm` · `Default` · `Lg` · `Icon` · `IconXs` · `IconSm` · `IconLg`
///
/// ## Example
///
/// ```rust
/// Button {
///     variant: ButtonVariant::Outline,
///     size: ButtonSize::Sm,
///     onclick: move |_| log::info!("clicked"),
///     "Small outline"
/// }
///
/// Button {
///     variant: ButtonVariant::Outline,
///     size: ButtonSize::IconSm,
///     aria_label: "Submit",
///     ArrowUpRightIcon {}
/// }
/// ```
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let class = format!(
        "{} {} {} {}",
        BASE_CLASSES,
        props.variant.classes(),
        props.size.classes(),
        props.class,
    );

    rsx! {
        button {
            r#type: "{props.r#type}",
            class: class,
            disabled: props.disabled,
            aria_label: props.aria_label,
            "data-slot": "button",
            "data-variant": props.variant.as_str(),
            "data-size": props.size.as_str(),
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}

use dioxus::prelude::*;
use primitives_core::toggle::BASE_CLASSES;

// Re-export core types so consumers don't need to depend on primitives-core
// directly.
pub use primitives_core::toggle::{ToggleSize, ToggleVariant};

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    /// Visual style of the toggle.
    #[props(default)]
    pub variant: ToggleVariant,

    /// Controls height and horizontal padding.
    #[props(default)]
    pub size: ToggleSize,

    /// Additional Tailwind classes merged onto the root element.
    #[props(default)]
    pub class: String,

    /// Maps to the HTML `disabled` attribute.
    #[props(default)]
    pub disabled: bool,

    /// Controlled pressed state. When provided the component is *controlled* —
    /// pair with `on_pressed_change` to update state.
    #[props(default)]
    pub pressed: Option<bool>,

    /// Initial pressed state for an *uncontrolled* toggle.
    #[props(default)]
    pub default_pressed: bool,

    /// Accessible label — recommended when the toggle only contains an icon.
    #[props(default)]
    pub aria_label: Option<String>,

    /// Callback fired when the pressed state changes. Receives the **new**
    /// pressed value.
    pub on_pressed_change: Option<EventHandler<bool>>,

    /// Toggle content (text, icons, etc.).
    pub children: Element,
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A two-state toggle button matching the shadcn/ui v4 API.
///
/// Supports both **controlled** (via `pressed` + `on_pressed_change`) and
/// **uncontrolled** (via `default_pressed`) modes.
///
/// ## Variants
///
/// `Default` · `Outline`
///
/// ## Sizes
///
/// `Sm` · `Default` · `Lg`
///
/// ## Example
///
/// ```rust
/// // Uncontrolled
/// Toggle {
///     default_pressed: false,
///     aria_label: "Toggle bold",
///     BoldIcon {}
/// }
///
/// // Controlled
/// Toggle {
///     variant: ToggleVariant::Outline,
///     pressed: is_active(),
///     on_pressed_change: move |next| is_active.set(next),
///     "Bold"
/// }
/// ```
#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    // Internal state — only meaningful in uncontrolled mode.
    let mut internal_pressed = use_signal(|| props.default_pressed);

    // Resolve effective pressed state: controlled takes priority.
    let is_pressed = props.pressed.unwrap_or_else(|| internal_pressed());

    let state = if is_pressed { "on" } else { "off" };

    let class = format!(
        "{} {} {} {}",
        BASE_CLASSES,
        props.variant.classes(),
        props.size.classes(),
        props.class,
    );

    rsx! {
        button {
            r#type: "button",
            class: class,
            disabled: props.disabled,
            aria_pressed: "{is_pressed}",
            aria_label: props.aria_label,
            "data-slot": "toggle",
            "data-state": state,
            "data-variant": props.variant.as_str(),
            "data-size": props.size.as_str(),
            onclick: move |_| {
                let next = !is_pressed;
                // Always update internal state (no-op when controlled, but
                // keeps the signal in sync if the consumer switches modes).
                internal_pressed.set(next);
                if let Some(handler) = &props.on_pressed_change {
                    handler.call(next);
                }
            },
            {props.children}
        }
    }
}

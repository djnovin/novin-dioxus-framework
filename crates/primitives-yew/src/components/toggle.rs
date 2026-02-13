use primitives_core::toggle::{BASE_CLASSES, ToggleSize, ToggleVariant};
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct ToggleProps {
    /// Visual style of the toggle.
    #[prop_or_default]
    pub variant: ToggleVariant,

    /// Controls height and horizontal padding.
    #[prop_or_default]
    pub size: ToggleSize,

    /// Additional Tailwind classes merged onto the root element.
    #[prop_or_default]
    pub class: String,

    /// Maps to the HTML `disabled` attribute.
    #[prop_or_default]
    pub disabled: bool,

    /// Controlled pressed state. When provided the component is *controlled* —
    /// pair with `on_pressed_change` to update state.
    #[prop_or_default]
    pub pressed: Option<bool>,

    /// Initial pressed state for an *uncontrolled* toggle.
    #[prop_or_default]
    pub default_pressed: bool,

    /// Accessible label — recommended when the toggle only contains an icon.
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// Callback fired when the pressed state changes. Receives the **new**
    /// pressed value.
    #[prop_or_default]
    pub on_pressed_change: Option<Callback<bool>>,

    /// Toggle content (text, icons, etc.).
    pub children: Children,
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
/// html! {
///     <Toggle default_pressed=false aria_label="Toggle bold">
///         <BoldIcon />
///     </Toggle>
/// }
///
/// // Controlled
/// html! {
///     <Toggle
///         variant={ToggleVariant::Outline}
///         pressed={*is_active}
///         on_pressed_change={Callback::from(move |next: bool| is_active.set(next))}
///     >
///         {"Bold"}
///     </Toggle>
/// }
/// ```
#[component(Toggle)]
pub fn toggle(props: &ToggleProps) -> Html {
    // Internal state — only meaningful in uncontrolled mode.
    let internal_pressed = use_state(|| props.default_pressed);

    // Resolve effective pressed state: controlled takes priority.
    let is_pressed = props.pressed.unwrap_or_else(|| *internal_pressed);

    let state = if is_pressed { "on" } else { "off" };

    let classes = format!(
        "{} {} {} {}",
        BASE_CLASSES,
        props.variant.classes(),
        props.size.classes(),
        props.class,
    );

    let data_variant = props.variant.as_str();
    let data_size = props.size.as_str();

    let onclick = {
        let internal_pressed = internal_pressed.clone();
        let on_pressed_change = props.on_pressed_change.clone();
        let current = is_pressed;
        Callback::from(move |_: MouseEvent| {
            let next = !current;
            // Always update internal state (no-op when controlled, but
            // keeps the state in sync if the consumer switches modes).
            internal_pressed.set(next);
            if let Some(ref handler) = on_pressed_change {
                handler.emit(next);
            }
        })
    };

    html! {
        <button
            type="button"
            class={classes}
            disabled={props.disabled}
            aria-pressed={is_pressed.to_string()}
            aria-label={props.aria_label.clone()}
            data-slot="toggle"
            data-state={state}
            data-variant={data_variant}
            data-size={data_size}
            onclick={onclick}
        >
            { for props.children.iter() }
        </button>
    }
}

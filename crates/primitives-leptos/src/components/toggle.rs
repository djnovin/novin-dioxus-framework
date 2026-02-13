use leptos::prelude::*;
use primitives_core::toggle::{BASE_CLASSES, ToggleSize, ToggleVariant};

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
/// view! {
///     <Toggle default_pressed=false aria_label="Toggle bold">
///         <BoldIcon />
///     </Toggle>
/// }
///
/// // Controlled
/// view! {
///     <Toggle
///         variant=ToggleVariant::Outline
///         pressed=is_active.get()
///         on_pressed_change=move |next| set_is_active.set(next)
///     >
///         "Bold"
///     </Toggle>
/// }
/// ```
#[component]
pub fn Toggle(
    /// Visual style of the toggle.
    #[prop(optional, into)]
    variant: ToggleVariant,
    /// Controls height and horizontal padding.
    #[prop(optional, into)]
    size: ToggleSize,
    /// Additional Tailwind classes merged onto the root element.
    #[prop(optional, into)]
    class: String,
    /// Maps to the HTML `disabled` attribute.
    #[prop(optional)]
    disabled: bool,
    /// Controlled pressed state. When provided the component is *controlled* —
    /// pair with `on_pressed_change` to update state.
    #[prop(optional, into)]
    pressed: Option<bool>,
    /// Initial pressed state for an *uncontrolled* toggle.
    #[prop(optional)]
    default_pressed: bool,
    /// Accessible label — recommended when the toggle only contains an icon.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Callback fired when the pressed state changes. Receives the **new**
    /// pressed value.
    #[prop(optional, into)]
    on_pressed_change: Option<Callback<bool>>,
    /// Toggle content (text, icons, etc.).
    children: Children,
) -> impl IntoView {
    // Internal state — only meaningful in uncontrolled mode.
    let (internal_pressed, set_internal_pressed) = signal(default_pressed);

    // Resolve effective pressed state: controlled takes priority.
    let is_pressed = move || pressed.unwrap_or_else(|| internal_pressed.get());

    let classes = format!(
        "{} {} {} {}",
        BASE_CLASSES,
        variant.classes(),
        size.classes(),
        class,
    );

    let data_variant = variant.as_str();
    let data_size = size.as_str();

    view! {
        <button
            type="button"
            class={classes}
            disabled={disabled}
            aria-pressed={move || is_pressed().to_string()}
            aria-label={aria_label}
            data-slot="toggle"
            data-state={move || if is_pressed() { "on" } else { "off" }}
            data-variant={data_variant}
            data-size={data_size}
            on:click=move |_| {
                let next = !is_pressed();
                // Always update internal state (no-op when controlled, but
                // keeps the signal in sync if the consumer switches modes).
                set_internal_pressed.set(next);
                if let Some(ref handler) = on_pressed_change {
                    handler.run(next);
                }
            }
        >
            {children()}
        </button>
    }
}

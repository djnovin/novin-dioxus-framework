use leptos::ev;
use leptos::prelude::*;
use primitives_core::input::BASE_CLASSES;

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A low-level input primitive matching the shadcn/ui v4 `<Input>`.
///
/// This is a fully accessible `<input>` element with the complete shadcn v4
/// class set baked in. Intended to be wrapped by higher-level `Field` / form
/// components.
///
/// ## Example
///
/// ```rust
/// view! {
///     <Input placeholder="Enter text" />
///
///     <Input
///         r#type="email"
///         placeholder="you@example.com"
///         required=true
///         on_input=move |evt| log::info!("value changed")
///     />
/// }
/// ```
#[component]
pub fn Input(
    /// Additional Tailwind classes merged onto the `<input>` element.
    #[prop(optional, into)]
    class: String,
    /// HTML input type (`"text"`, `"email"`, `"password"`, `"number"`, `"file"`, etc.).
    /// Defaults to `"text"`.
    #[prop(optional, into, default = "text".to_string())]
    r#type: String,
    /// Placeholder text shown when the input is empty.
    #[prop(optional, into)]
    placeholder: String,
    /// Controlled value. When provided the input is *controlled* — pair with
    /// `on_input` to update state.
    #[prop(optional, into)]
    value: Option<String>,
    /// Default (initial) value for an *uncontrolled* input.
    #[prop(optional, into)]
    default_value: Option<String>,
    /// The `name` attribute, used for form submission.
    #[prop(optional, into)]
    name: Option<String>,
    /// Maps to the HTML `disabled` attribute.
    #[prop(optional)]
    disabled: bool,
    /// Maps to the HTML `readonly` attribute.
    #[prop(optional)]
    readonly: bool,
    /// Maps to the HTML `required` attribute.
    #[prop(optional)]
    required: bool,
    /// Sets `aria-invalid` on the element, which activates the destructive
    /// ring styles from the base classes.
    #[prop(optional, into)]
    aria_invalid: Option<bool>,
    /// Accessible label for screen readers.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Associates the input with a `<label>` via id.
    #[prop(optional, into)]
    id: Option<String>,
    /// HTML `autocomplete` attribute (`"off"`, `"email"`, `"current-password"`, etc.).
    #[prop(optional, into)]
    autocomplete: Option<String>,
    /// Maximum character length.
    #[prop(optional, into)]
    maxlength: Option<i64>,
    /// Minimum character length.
    #[prop(optional, into)]
    minlength: Option<i64>,
    /// Regex pattern for validation.
    #[prop(optional, into)]
    pattern: Option<String>,
    /// Fires on every keystroke / input change.
    #[prop(optional, into)]
    on_input: Option<Callback<ev::Event>>,
    /// Fires when the value is committed (blur / Enter).
    #[prop(optional, into)]
    on_change: Option<Callback<ev::Event>>,
    /// Fires when the input receives focus.
    #[prop(optional, into)]
    on_focus: Option<Callback<ev::FocusEvent>>,
    /// Fires when the input loses focus.
    #[prop(optional, into)]
    on_blur: Option<Callback<ev::FocusEvent>>,
) -> impl IntoView {
    let classes = format!("{} {}", BASE_CLASSES, class);

    let aria_invalid_str = aria_invalid.map(|v| v.to_string());

    // Resolve the effective value attribute — prefer controlled `value` over
    // `default_value`.
    let effective_value = value.or(default_value);

    view! {
        <input
            type={r#type}
            class={classes}
            placeholder={placeholder}
            disabled={disabled}
            readonly={readonly}
            required={required}
            data-slot="input"
            value={effective_value}
            name={name}
            id={id}
            aria-invalid={aria_invalid_str}
            aria-label={aria_label}
            autocomplete={autocomplete}
            maxlength={maxlength.map(|v| v.to_string())}
            minlength={minlength.map(|v| v.to_string())}
            pattern={pattern}
            on:input=move |evt| {
                if let Some(ref handler) = on_input {
                    handler.run(evt);
                }
            }
            on:change=move |evt| {
                if let Some(ref handler) = on_change {
                    handler.run(evt);
                }
            }
            on:focus=move |evt| {
                if let Some(ref handler) = on_focus {
                    handler.run(evt);
                }
            }
            on:blur=move |evt| {
                if let Some(ref handler) = on_blur {
                    handler.run(evt);
                }
            }
        />
    }
}

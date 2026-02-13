use leptos::ev;
use leptos::prelude::*;
use primitives_core::text_area::BASE_CLASSES;

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A textarea component matching the shadcn/ui v4 API.
///
/// This is a fully accessible `<textarea>` element with the complete shadcn v4
/// class set baked in. Intended to be wrapped by higher-level `Field` / form
/// components.
///
/// Uses `primitives_core::text_area::BASE_CLASSES` for the invariant Tailwind
/// classes.
///
/// ## Example
///
/// ```rust
/// view! {
///     <TextArea placeholder="Enter your message…" />
///
///     <TextArea
///         placeholder="Write something…"
///         rows=5
///         required=true
///         on_input=move |evt| log::info!("value changed")
///     />
/// }
/// ```
#[component]
pub fn TextArea(
    /// Additional Tailwind classes merged onto the `<textarea>` element.
    #[prop(optional, into)]
    class: String,
    /// Placeholder text shown when the textarea is empty.
    #[prop(optional, into)]
    placeholder: String,
    /// Controlled value. When provided the textarea is *controlled* — pair with
    /// `on_input` to update state.
    #[prop(optional, into)]
    value: Option<String>,
    /// Default (initial) value for an *uncontrolled* textarea.
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
    /// Associates the textarea with a `<label>` via id.
    #[prop(optional, into)]
    id: Option<String>,
    /// The number of visible text rows.
    #[prop(optional, into)]
    rows: Option<i64>,
    /// The number of visible text columns.
    #[prop(optional, into)]
    cols: Option<i64>,
    /// Maximum character length.
    #[prop(optional, into)]
    maxlength: Option<i64>,
    /// Minimum character length.
    #[prop(optional, into)]
    minlength: Option<i64>,
    /// Fires on every keystroke / input change.
    #[prop(optional, into)]
    on_input: Option<Callback<ev::Event>>,
    /// Fires when the value is committed (blur / Enter).
    #[prop(optional, into)]
    on_change: Option<Callback<ev::Event>>,
    /// Fires when the textarea receives focus.
    #[prop(optional, into)]
    on_focus: Option<Callback<ev::FocusEvent>>,
    /// Fires when the textarea loses focus.
    #[prop(optional, into)]
    on_blur: Option<Callback<ev::FocusEvent>>,
    /// Fires on key down.
    #[prop(optional, into)]
    on_keydown: Option<Callback<ev::KeyboardEvent>>,
    /// Fires on key up.
    #[prop(optional, into)]
    on_keyup: Option<Callback<ev::KeyboardEvent>>,
) -> impl IntoView {
    let classes = format!("{} {}", BASE_CLASSES, class);

    let aria_invalid_str = aria_invalid.map(|v| v.to_string());

    // Resolve the effective value — prefer controlled `value` over `default_value`.
    let effective_value = value.or(default_value);

    view! {
        <textarea
            class={classes}
            placeholder={placeholder}
            disabled={disabled}
            readonly={readonly}
            required={required}
            data-slot="textarea"
            name={name}
            id={id}
            aria-invalid={aria_invalid_str}
            aria-label={aria_label}
            rows={rows.map(|v| v.to_string())}
            cols={cols.map(|v| v.to_string())}
            maxlength={maxlength.map(|v| v.to_string())}
            minlength={minlength.map(|v| v.to_string())}
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
            on:keydown=move |evt| {
                if let Some(ref handler) = on_keydown {
                    handler.run(evt);
                }
            }
            on:keyup=move |evt| {
                if let Some(ref handler) = on_keyup {
                    handler.run(evt);
                }
            }
        >
            {effective_value}
        </textarea>
    }
}

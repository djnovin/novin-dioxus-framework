use dioxus::prelude::*;
use primitives_core::input::BASE_CLASSES;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// Additional Tailwind classes merged onto the `<input>` element.
    #[props(default)]
    pub class: String,

    /// HTML input type (`"text"`, `"email"`, `"password"`, `"number"`, `"file"`, etc.).
    /// Defaults to `"text"`.
    #[props(default = "text".to_string())]
    pub r#type: String,

    /// Placeholder text shown when the input is empty.
    #[props(default)]
    pub placeholder: String,

    /// Controlled value. When provided the input is *controlled* — pair with
    /// `oninput` to update state.
    #[props(default)]
    pub value: Option<String>,

    /// Default (initial) value for an *uncontrolled* input.
    #[props(default)]
    pub default_value: Option<String>,

    /// The `name` attribute, used for form submission.
    #[props(default)]
    pub name: Option<String>,

    /// Maps to the HTML `disabled` attribute.
    #[props(default)]
    pub disabled: bool,

    /// Maps to the HTML `readonly` attribute.
    #[props(default)]
    pub readonly: bool,

    /// Maps to the HTML `required` attribute.
    #[props(default)]
    pub required: bool,

    /// Sets `aria-invalid` on the element, which activates the destructive
    /// ring styles from the base classes.
    #[props(default)]
    pub aria_invalid: Option<bool>,

    /// Accessible label for screen readers.
    #[props(default)]
    pub aria_label: Option<String>,

    /// Associates the input with a `<label>` via id.
    #[props(default)]
    pub id: Option<String>,

    /// HTML `autocomplete` attribute (`"off"`, `"email"`, `"current-password"`, etc.).
    #[props(default)]
    pub autocomplete: Option<String>,

    /// Maximum character length.
    #[props(default)]
    pub maxlength: Option<i64>,

    /// Minimum character length.
    #[props(default)]
    pub minlength: Option<i64>,

    /// Regex pattern for validation.
    #[props(default)]
    pub pattern: Option<String>,

    /// Fires on every keystroke / input change.
    pub oninput: Option<EventHandler<FormEvent>>,

    /// Fires when the value is committed (blur / Enter).
    pub onchange: Option<EventHandler<FormEvent>>,

    /// Fires when the input receives focus.
    pub onfocus: Option<EventHandler<FocusEvent>>,

    /// Fires when the input loses focus.
    pub onblur: Option<EventHandler<FocusEvent>>,
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A low-level input primitive matching the shadcn/ui v4 `<Input>`.
///
/// This is an unstyled-by-default, fully accessible `<input>` element with the
/// complete shadcn v4 class set baked in. Intended to be wrapped by
/// higher-level `Field` / form components.
///
/// ## Example
///
/// ```rust
/// Input {
///     placeholder: "Enter text",
/// }
///
/// Input {
///     r#type: "email",
///     placeholder: "you@example.com",
///     required: true,
///     oninput: move |evt: FormEvent| {
///         log::info!("value: {}", evt.value());
///     },
/// }
/// ```
#[component]
pub fn Input(props: InputProps) -> Element {
    let class = format!("{} {}", BASE_CLASSES, props.class);

    rsx! {
        input {
            r#type: "{props.r#type}",
            class: class,
            placeholder: "{props.placeholder}",
            disabled: props.disabled,
            readonly: props.readonly,
            required: props.required,
            "data-slot": "input",

            // Optional attributes — only rendered when `Some`.
            value: props.value,
            initial_value: props.default_value,
            name: props.name,
            id: props.id,
            aria_invalid: props.aria_invalid.map(|v| v.to_string()),
            aria_label: props.aria_label,
            autocomplete: props.autocomplete,
            maxlength: props.maxlength,
            minlength: props.minlength,
            pattern: props.pattern,

            // Event handlers — only attached when provided.
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
                    handler.call(evt);
                }
            },
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            },
            onfocus: move |evt| {
                if let Some(handler) = &props.onfocus {
                    handler.call(evt);
                }
            },
            onblur: move |evt| {
                if let Some(handler) = &props.onblur {
                    handler.call(evt);
                }
            },
        }
    }
}

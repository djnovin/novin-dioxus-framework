use primitives_core::input::BASE_CLASSES;
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct InputProps {
    /// Additional Tailwind classes merged onto the `<input>` element.
    #[prop_or_default]
    pub class: String,

    /// HTML input type (`"text"`, `"email"`, `"password"`, `"number"`, `"file"`, etc.).
    /// Defaults to `"text"`.
    #[prop_or("text".to_string())]
    pub r#type: String,

    /// Placeholder text shown when the input is empty.
    #[prop_or_default]
    pub placeholder: String,

    /// Controlled value. When provided the input is *controlled* — pair with
    /// `oninput` to update state.
    #[prop_or_default]
    pub value: Option<String>,

    /// Default (initial) value for an *uncontrolled* input.
    #[prop_or_default]
    pub default_value: Option<String>,

    /// The `name` attribute, used for form submission.
    #[prop_or_default]
    pub name: Option<String>,

    /// Maps to the HTML `disabled` attribute.
    #[prop_or_default]
    pub disabled: bool,

    /// Maps to the HTML `readonly` attribute.
    #[prop_or_default]
    pub readonly: bool,

    /// Maps to the HTML `required` attribute.
    #[prop_or_default]
    pub required: bool,

    /// Sets `aria-invalid` on the element, which activates the destructive
    /// ring styles from the base classes.
    #[prop_or_default]
    pub aria_invalid: Option<bool>,

    /// Accessible label for screen readers.
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// Associates the input with a `<label>` via id.
    #[prop_or_default]
    pub id: Option<String>,

    /// HTML `autocomplete` attribute (`"off"`, `"email"`, `"current-password"`, etc.).
    #[prop_or_default]
    pub autocomplete: Option<String>,

    /// Maximum character length.
    #[prop_or_default]
    pub maxlength: Option<i64>,

    /// Minimum character length.
    #[prop_or_default]
    pub minlength: Option<i64>,

    /// Regex pattern for validation.
    #[prop_or_default]
    pub pattern: Option<String>,

    /// Fires on every keystroke / input change.
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,

    /// Fires when the value is committed (blur / Enter).
    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,

    /// Fires when the input receives focus.
    #[prop_or_default]
    pub onfocus: Option<Callback<FocusEvent>>,

    /// Fires when the input loses focus.
    #[prop_or_default]
    pub onblur: Option<Callback<FocusEvent>>,
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A low-level input primitive matching the shadcn/ui v4 `<Input>`.
///
/// This is a fully accessible `<input>` element with the complete shadcn v4
/// class set baked in. Intended to be wrapped by higher-level `Field` / form
/// components.
///
/// Uses `primitives_core::input::BASE_CLASSES` for the invariant Tailwind
/// classes.
///
/// ## Example
///
/// ```rust
/// html! {
///     <Input placeholder="Enter text" />
///
///     <Input
///         r#type="email"
///         placeholder="you@example.com"
///         required=true
///         oninput={Callback::from(|e: InputEvent| {
///             // handle input
///         })}
///     />
/// }
/// ```
#[component(Input)]
pub fn input(props: &InputProps) -> Html {
    let classes = format!("{} {}", BASE_CLASSES, props.class);

    let aria_invalid_str = props.aria_invalid.map(|v| v.to_string());

    // Resolve the effective value attribute — prefer controlled `value` over
    // `default_value`.
    let effective_value = props.value.clone().or_else(|| props.default_value.clone());

    let oninput = {
        let handler = props.oninput.clone();
        Callback::from(move |evt: InputEvent| {
            if let Some(ref cb) = handler {
                cb.emit(evt);
            }
        })
    };

    let onchange = {
        let handler = props.onchange.clone();
        Callback::from(move |evt: Event| {
            if let Some(ref cb) = handler {
                cb.emit(evt);
            }
        })
    };

    let onfocus = {
        let handler = props.onfocus.clone();
        Callback::from(move |evt: FocusEvent| {
            if let Some(ref cb) = handler {
                cb.emit(evt);
            }
        })
    };

    let onblur = {
        let handler = props.onblur.clone();
        Callback::from(move |evt: FocusEvent| {
            if let Some(ref cb) = handler {
                cb.emit(evt);
            }
        })
    };

    html! {
        <input
            type={props.r#type.clone()}
            class={classes}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            readonly={props.readonly}
            required={props.required}
            data-slot="input"
            value={effective_value}
            name={props.name.clone()}
            id={props.id.clone()}
            aria-invalid={aria_invalid_str}
            aria-label={props.aria_label.clone()}
            autocomplete={props.autocomplete.clone()}
            maxlength={props.maxlength.map(|v| v.to_string())}
            minlength={props.minlength.map(|v| v.to_string())}
            pattern={props.pattern.clone()}
            oninput={oninput}
            onchange={onchange}
            onfocus={onfocus}
            onblur={onblur}
        />
    }
}

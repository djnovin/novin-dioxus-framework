use primitives_core::text_area::BASE_CLASSES;
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct TextAreaProps {
    /// Additional Tailwind classes merged onto the `<textarea>` element.
    #[prop_or_default]
    pub class: String,

    /// Placeholder text shown when the textarea is empty.
    #[prop_or_default]
    pub placeholder: String,

    /// Controlled value. When provided the textarea is *controlled* — pair with
    /// `oninput` to update state.
    #[prop_or_default]
    pub value: Option<String>,

    /// Default (initial) value for an *uncontrolled* textarea.
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

    /// Associates the textarea with a `<label>` via id.
    #[prop_or_default]
    pub id: Option<String>,

    /// The number of visible text rows.
    #[prop_or_default]
    pub rows: Option<u32>,

    /// The number of visible text columns.
    #[prop_or_default]
    pub cols: Option<u32>,

    /// Maximum character length.
    #[prop_or_default]
    pub maxlength: Option<u32>,

    /// Minimum character length.
    #[prop_or_default]
    pub minlength: Option<u32>,

    /// Fires on every keystroke / input change.
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,

    /// Fires when the value is committed (blur / Enter).
    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,

    /// Fires when the textarea receives focus.
    #[prop_or_default]
    pub onfocus: Option<Callback<FocusEvent>>,

    /// Fires when the textarea loses focus.
    #[prop_or_default]
    pub onblur: Option<Callback<FocusEvent>>,

    /// Fires on key down.
    #[prop_or_default]
    pub onkeydown: Option<Callback<KeyboardEvent>>,

    /// Fires on key up.
    #[prop_or_default]
    pub onkeyup: Option<Callback<KeyboardEvent>>,
}

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
/// html! {
///     <TextArea placeholder="Enter your message…" />
///
///     <TextArea
///         placeholder="Write something…"
///         rows={5}
///         required=true
///         oninput={Callback::from(|e: InputEvent| {
///             // handle input
///         })}
///     />
/// }
/// ```
#[component(TextArea)]
pub fn text_area(props: &TextAreaProps) -> Html {
    let classes = format!("{} {}", BASE_CLASSES, props.class);

    let aria_invalid_str = props.aria_invalid.map(|v| v.to_string());

    // Resolve the effective value — prefer controlled `value` over `default_value`.
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

    let onkeydown = {
        let handler = props.onkeydown.clone();
        Callback::from(move |evt: KeyboardEvent| {
            if let Some(ref cb) = handler {
                cb.emit(evt);
            }
        })
    };

    let onkeyup = {
        let handler = props.onkeyup.clone();
        Callback::from(move |evt: KeyboardEvent| {
            if let Some(ref cb) = handler {
                cb.emit(evt);
            }
        })
    };

    html! {
        <textarea
            class={classes}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            readonly={props.readonly}
            required={props.required}
            data-slot="textarea"
            value={effective_value}
            name={props.name.clone()}
            id={props.id.clone()}
            aria-invalid={aria_invalid_str}
            aria-label={props.aria_label.clone()}
            rows={props.rows.map(|v| v.to_string())}
            cols={props.cols.map(|v| v.to_string())}
            maxlength={props.maxlength.map(|v| v.to_string())}
            minlength={props.minlength.map(|v| v.to_string())}
            oninput={oninput}
            onchange={onchange}
            onfocus={onfocus}
            onblur={onblur}
            onkeydown={onkeydown}
            onkeyup={onkeyup}
        />
    }
}

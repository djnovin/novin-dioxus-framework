use primitives_core::button::{BASE_CLASSES, ButtonSize, ButtonVariant};
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    /// Visual style of the button.
    #[prop_or_default]
    pub variant: ButtonVariant,

    /// Controls height, padding, and icon sizing.
    #[prop_or_default]
    pub size: ButtonSize,

    /// Additional Tailwind classes merged onto the root element.
    #[prop_or_default]
    pub class: String,

    /// Maps to the HTML `disabled` attribute.
    #[prop_or_default]
    pub disabled: bool,

    /// Accessible label — required for icon-only buttons.
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// HTML button type attribute (`"button"`, `"submit"`, `"reset"`).
    /// Defaults to `"button"`.
    #[prop_or("button".to_string())]
    pub r#type: String,

    /// Click handler.
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Button content (text, icons, spinners, etc.).
    pub children: Children,
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
/// html! {
///     <Button
///         variant={ButtonVariant::Outline}
///         size={ButtonSize::Sm}
///         onclick={Callback::from(|_| log::info!("clicked"))}
///     >
///         {"Small outline"}
///     </Button>
///
///     <Button
///         variant={ButtonVariant::Outline}
///         size={ButtonSize::IconSm}
///         aria_label="Submit"
///     >
///         <ArrowUpRightIcon />
///     </Button>
/// }
/// ```
#[component(Button)]
pub fn button(props: &ButtonProps) -> Html {
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
        let handler = props.onclick.clone();
        Callback::from(move |evt: MouseEvent| {
            if let Some(ref cb) = handler {
                cb.emit(evt);
            }
        })
    };

    html! {
        <button
            type={props.r#type.clone()}
            class={classes}
            disabled={props.disabled}
            aria-label={props.aria_label.clone()}
            data-slot="button"
            data-variant={data_variant}
            data-size={data_size}
            onclick={onclick}
        >
            { for props.children.iter() }
        </button>
    }
}

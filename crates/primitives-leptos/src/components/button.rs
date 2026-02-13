use leptos::ev;
use leptos::prelude::*;
use primitives_core::button::{BASE_CLASSES, ButtonSize, ButtonVariant};

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
/// view! {
///     <Button
///         variant=ButtonVariant::Outline
///         size=ButtonSize::Sm
///         on:click=move |_| log::info!("clicked")
///     >
///         "Small outline"
///     </Button>
/// }
/// ```
#[component]
pub fn Button(
    /// Visual style of the button.
    #[prop(optional, into)]
    variant: ButtonVariant,
    /// Controls height, padding, and icon sizing.
    #[prop(optional, into)]
    size: ButtonSize,
    /// Additional Tailwind classes merged onto the root element.
    #[prop(optional, into)]
    class: String,
    /// Maps to the HTML `disabled` attribute.
    #[prop(optional)]
    disabled: bool,
    /// Accessible label — required for icon-only buttons.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// HTML button type attribute. Defaults to `"button"`.
    #[prop(optional, into, default = "button".to_string())]
    r#type: String,
    /// Click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// Button content (text, icons, spinners, etc.).
    children: Children,
) -> impl IntoView {
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
            type={r#type}
            class={classes}
            disabled={disabled}
            aria-label={aria_label}
            data-slot="button"
            data-variant={data_variant}
            data-size={data_size}
            on:click=move |evt| {
                if let Some(ref handler) = on_click {
                    handler.run(evt);
                }
            }
        >
            {children()}
        </button>
    }
}

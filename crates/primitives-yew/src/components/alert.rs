use primitives_core::alert::{AlertVariant, BASE_CLASSES};
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct AlertProps {
    /// Visual style of the alert.
    #[prop_or_default]
    pub variant: AlertVariant,

    /// Additional Tailwind classes merged onto the root element.
    #[prop_or_default]
    pub class: String,

    /// Alert content (text, icons, etc.).
    pub children: Children,
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// An alert component matching the shadcn/ui v4 API.
///
/// ## Variants
///
/// `Info` (default) · `Success` · `Warning` · `Error`
///
/// ## Example
///
/// ```rust
/// html! {
///     <Alert variant={AlertVariant::Success}>
///         {"Operation completed successfully!"}
///     </Alert>
///
///     <Alert variant={AlertVariant::Error} class="mt-4">
///         {"Something went wrong. Please try again."}
///     </Alert>
/// }
/// ```
#[component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    let classes = format!(
        "{} {} {}",
        BASE_CLASSES,
        props.variant.classes(),
        props.class
    );

    let data_variant = props.variant.as_str();

    html! {
        <div
            class={classes}
            role="alert"
            data-slot="alert"
            data-variant={data_variant}
        >
            { for props.children.iter() }
        </div>
    }
}

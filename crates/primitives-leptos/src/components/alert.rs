use leptos::prelude::*;
use primitives_core::alert::{AlertVariant, BASE_CLASSES};

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// An alert component matching the shadcn/ui v4 API.
///
/// ## Variants
///
/// `Info` · `Success` · `Warning` · `Error`
///
/// ## Example
///
/// ```rust
/// view! {
///     <Alert variant=AlertVariant::Success>
///         "Operation completed successfully!"
///     </Alert>
///
///     <Alert variant=AlertVariant::Error class="mt-4">
///         "Something went wrong. Please try again."
///     </Alert>
/// }
/// ```
#[component]
pub fn Alert(
    /// Visual style of the alert.
    #[prop(optional, into)]
    variant: AlertVariant,
    /// Additional Tailwind classes merged onto the root element.
    #[prop(optional, into)]
    class: String,
    /// Alert content (text, icons, etc.).
    children: Children,
) -> impl IntoView {
    let classes = format!("{} {} {}", BASE_CLASSES, variant.classes(), class);

    let data_variant = variant.as_str();

    view! {
        <div
            class={classes}
            role="alert"
            data-slot="alert"
            data-variant={data_variant}
        >
            {children()}
        </div>
    }
}

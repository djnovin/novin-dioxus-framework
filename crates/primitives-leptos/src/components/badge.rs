use leptos::prelude::*;
use primitives_core::badge::{BASE_CLASSES, BadgeVariant};

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A badge component matching the shadcn/ui v4 API.
///
/// ## Variants
///
/// `Default` · `Secondary` · `Outline` · `Destructive`
///
/// ## Example
///
/// ```rust
/// view! {
///     <Badge variant=BadgeVariant::Secondary>
///         "New"
///     </Badge>
/// }
/// ```
#[component]
pub fn Badge(
    /// Visual style of the badge.
    #[prop(optional, into)]
    variant: BadgeVariant,
    /// Additional Tailwind classes merged onto the root element.
    #[prop(optional, into)]
    class: String,
    /// Badge content (text, icons, etc.).
    children: Children,
) -> impl IntoView {
    let classes = format!("{} {} {}", BASE_CLASSES, variant.classes(), class);

    let data_variant = variant.as_str();

    view! {
        <span
            class={classes}
            data-slot="badge"
            data-variant={data_variant}
        >
            {children()}
        </span>
    }
}

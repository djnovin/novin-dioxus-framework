use primitives_core::badge::{BASE_CLASSES, BadgeVariant};
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct BadgeProps {
    /// Visual style of the badge.
    #[prop_or_default]
    pub variant: BadgeVariant,

    /// Additional Tailwind classes merged onto the root element.
    #[prop_or_default]
    pub class: String,

    /// Badge content (text, icons, etc.).
    pub children: Children,
}

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
/// html! {
///     <Badge variant={BadgeVariant::Secondary}>
///         {"New"}
///     </Badge>
///
///     <Badge variant={BadgeVariant::Destructive} class="ml-2">
///         {"Error"}
///     </Badge>
/// }
/// ```
#[component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let classes = format!(
        "{} {} {}",
        BASE_CLASSES,
        props.variant.classes(),
        props.class
    );

    let data_variant = props.variant.as_str();

    html! {
        <span
            class={classes}
            data-slot="badge"
            data-variant={data_variant}
        >
            { for props.children.iter() }
        </span>
    }
}

use primitives_core::link::BASE_CLASSES;
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    /// The URL the link points to.
    pub href: String,

    /// Additional Tailwind classes merged onto the root element.
    #[prop_or_default]
    pub class: String,

    /// Link target attribute (`"_blank"`, `"_self"`, etc.).
    #[prop_or_default]
    pub target: Option<String>,

    /// Link rel attribute (e.g. `"noopener noreferrer"`). Automatically set to
    /// `"noopener noreferrer"` when `target` is `"_blank"` unless explicitly
    /// overridden.
    #[prop_or_default]
    pub rel: Option<String>,

    /// Accessible label for screen readers.
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// Link content (text, icons, etc.).
    pub children: Children,
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A link component that renders a styled `<a>` element.
///
/// Uses `primitives_core::link::BASE_CLASSES` for the invariant Tailwind
/// classes. Automatically adds `rel="noopener noreferrer"` when `target` is
/// `"_blank"` for security, unless a custom `rel` value is provided.
///
/// ## Example
///
/// ```rust
/// html! {
///     <Link href="https://example.com">
///         {"Visit Example"}
///     </Link>
///
///     <Link href="https://example.com" target="_blank" class="font-bold">
///         {"Opens in new tab"}
///     </Link>
/// }
/// ```
#[component(Link)]
pub fn link(props: &LinkProps) -> Html {
    let classes = format!("{} {}", BASE_CLASSES, props.class);

    // Auto-apply rel="noopener noreferrer" for _blank targets when no custom
    // rel is specified.
    let effective_rel = props.rel.clone().or_else(|| {
        props
            .target
            .as_deref()
            .filter(|t| *t == "_blank")
            .map(|_| "noopener noreferrer".to_string())
    });

    html! {
        <a
            class={classes}
            href={props.href.clone()}
            target={props.target.clone()}
            rel={effective_rel}
            aria-label={props.aria_label.clone()}
            data-slot="link"
        >
            { for props.children.iter() }
        </a>
    }
}

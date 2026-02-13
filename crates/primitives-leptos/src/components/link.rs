use leptos::prelude::*;
use primitives_core::link::BASE_CLASSES;

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
/// view! {
///     <Link href="https://example.com">
///         "Visit Example"
///     </Link>
///
///     <Link href="https://example.com" target="_blank" class="font-bold">
///         "Opens in new tab"
///     </Link>
/// }
/// ```
#[component]
pub fn Link(
    /// The URL the link points to.
    #[prop(into)]
    href: String,
    /// Additional Tailwind classes merged onto the root element.
    #[prop(optional, into)]
    class: String,
    /// Link target attribute (`"_blank"`, `"_self"`, etc.).
    #[prop(optional, into)]
    target: Option<String>,
    /// Link rel attribute. Automatically set to `"noopener noreferrer"` when
    /// `target` is `"_blank"` unless explicitly overridden.
    #[prop(optional, into)]
    rel: Option<String>,
    /// Accessible label for screen readers.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Link content (text, icons, etc.).
    children: Children,
) -> impl IntoView {
    let classes = format!("{} {}", BASE_CLASSES, class);

    // Auto-apply rel="noopener noreferrer" for _blank targets when no custom
    // rel is specified.
    let effective_rel = rel.or_else(|| {
        target
            .as_deref()
            .filter(|t| *t == "_blank")
            .map(|_| "noopener noreferrer".to_string())
    });

    view! {
        <a
            class={classes}
            href={href}
            target={target}
            rel={effective_rel}
            aria-label={aria_label}
            data-slot="link"
        >
            {children()}
        </a>
    }
}

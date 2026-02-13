use leptos::prelude::*;
use primitives_core::card::BASE_CLASSES;

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A card component that provides a styled container with padding, border,
/// rounded corners, and a subtle shadow. Uses `primitives_core::card::BASE_CLASSES`
/// for the invariant Tailwind classes.
///
/// ## Example
///
/// ```rust
/// view! {
///     <Card>
///         <h3>"Card Title"</h3>
///         <p>"Some card content goes here."</p>
///     </Card>
///
///     <Card class="max-w-sm">
///         <h3>"Constrained Card"</h3>
///         <p>"This card has a max width."</p>
///     </Card>
/// }
/// ```
#[component]
pub fn Card(
    /// Additional Tailwind classes merged onto the root element.
    #[prop(optional, into)]
    class: String,
    /// Card content (headings, text, buttons, etc.).
    children: Children,
) -> impl IntoView {
    let classes = format!("{} {}", BASE_CLASSES, class);

    view! {
        <div
            class={classes}
            data-slot="card"
        >
            {children()}
        </div>
    }
}

use primitives_core::card::BASE_CLASSES;
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct CardProps {
    /// Additional Tailwind classes merged onto the root element.
    #[prop_or_default]
    pub class: String,

    /// Card content (headings, text, buttons, etc.).
    pub children: Children,
}

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
/// html! {
///     <Card>
///         <h3>{"Card Title"}</h3>
///         <p>{"Some card content goes here."}</p>
///     </Card>
///
///     <Card class="max-w-sm">
///         <h3>{"Constrained Card"}</h3>
///         <p>{"This card has a max width."}</p>
///     </Card>
/// }
/// ```
#[component(Card)]
pub fn card(props: &CardProps) -> Html {
    let classes = format!("{} {}", BASE_CLASSES, props.class);

    html! {
        <div
            class={classes}
            data-slot="card"
        >
            { for props.children.iter() }
        </div>
    }
}

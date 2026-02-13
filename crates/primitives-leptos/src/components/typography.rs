use leptos::prelude::*;
use primitives_core::typography::TypographyVariant;

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A typography component that renders the appropriate semantic HTML element
/// based on the variant. Uses `primitives_core::typography::TypographyVariant`
/// for shared class and tag mapping logic.
///
/// ## Variants
///
/// `H1` · `H2` · `H3` · `H4` · `H5` · `H6` · `Subtitle1` · `Subtitle2` ·
/// `Body1` · `Body2` · `Caption` · `Overline` · `Blockquote` · `Muted`
///
/// ## Example
///
/// ```rust
/// view! {
///     <Typography variant=TypographyVariant::H1>
///         "Hello World"
///     </Typography>
///
///     <Typography variant=TypographyVariant::Muted class="mt-2">
///         "Some muted helper text"
///     </Typography>
/// }
/// ```
#[component]
pub fn Typography(
    /// The typography variant that determines the semantic HTML tag and styling.
    #[prop(optional, into)]
    variant: TypographyVariant,
    /// Additional Tailwind classes merged onto the root element.
    #[prop(optional, into)]
    class: String,
    /// Typography content (text, inline elements, etc.).
    children: Children,
) -> impl IntoView {
    let variant_classes = variant.classes();
    let classes = format!("{} {}", variant_classes, class);

    // Leptos doesn't support dynamic tag names directly in the `view!` macro,
    // so we match on the variant and render the correct semantic element.
    match variant {
        TypographyVariant::H1 => view! {
            <h1 class={classes}>{children()}</h1>
        }
        .into_any(),
        TypographyVariant::H2 => view! {
            <h2 class={classes}>{children()}</h2>
        }
        .into_any(),
        TypographyVariant::H3 => view! {
            <h3 class={classes}>{children()}</h3>
        }
        .into_any(),
        TypographyVariant::H4 => view! {
            <h4 class={classes}>{children()}</h4>
        }
        .into_any(),
        TypographyVariant::H5 => view! {
            <h5 class={classes}>{children()}</h5>
        }
        .into_any(),
        TypographyVariant::H6 | TypographyVariant::Subtitle1 | TypographyVariant::Subtitle2 => {
            view! {
                <h6 class={classes}>{children()}</h6>
            }
            .into_any()
        }
        TypographyVariant::Blockquote => view! {
            <blockquote class={classes}>{children()}</blockquote>
        }
        .into_any(),
        TypographyVariant::Body1
        | TypographyVariant::Body2
        | TypographyVariant::Caption
        | TypographyVariant::Overline
        | TypographyVariant::Muted => view! {
            <p class={classes}>{children()}</p>
        }
        .into_any(),
    }
}

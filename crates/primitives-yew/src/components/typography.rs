use primitives_core::typography::TypographyVariant;
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct TypographyProps {
    /// The typography variant that determines the semantic HTML tag and styling.
    #[prop_or_default]
    pub variant: TypographyVariant,

    /// Additional Tailwind classes merged onto the root element.
    #[prop_or_default]
    pub class: String,

    /// Typography content (text, inline elements, etc.).
    pub children: Children,
}

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
/// html! {
///     <Typography variant={TypographyVariant::H1}>
///         {"Hello World"}
///     </Typography>
///
///     <Typography variant={TypographyVariant::Muted} class="mt-2">
///         {"Some muted helper text"}
///     </Typography>
/// }
/// ```
#[component(Typography)]
pub fn typography(props: &TypographyProps) -> Html {
    let variant_classes = props.variant.classes();
    let classes = format!("{} {}", variant_classes, props.class);

    // Yew doesn't support dynamic tag names directly in the `html!` macro,
    // so we match on the variant and render the correct semantic element.
    match props.variant {
        TypographyVariant::H1 => html! {
            <h1 class={classes}>
                { for props.children.iter() }
            </h1>
        },
        TypographyVariant::H2 => html! {
            <h2 class={classes}>
                { for props.children.iter() }
            </h2>
        },
        TypographyVariant::H3 => html! {
            <h3 class={classes}>
                { for props.children.iter() }
            </h3>
        },
        TypographyVariant::H4 => html! {
            <h4 class={classes}>
                { for props.children.iter() }
            </h4>
        },
        TypographyVariant::H5 => html! {
            <h5 class={classes}>
                { for props.children.iter() }
            </h5>
        },
        TypographyVariant::H6 | TypographyVariant::Subtitle1 | TypographyVariant::Subtitle2 => {
            html! {
                <h6 class={classes}>
                    { for props.children.iter() }
                </h6>
            }
        }
        TypographyVariant::Blockquote => html! {
            <blockquote class={classes}>
                { for props.children.iter() }
            </blockquote>
        },
        TypographyVariant::Body1
        | TypographyVariant::Body2
        | TypographyVariant::Caption
        | TypographyVariant::Overline
        | TypographyVariant::Muted => html! {
            <p class={classes}>
                { for props.children.iter() }
            </p>
        },
    }
}

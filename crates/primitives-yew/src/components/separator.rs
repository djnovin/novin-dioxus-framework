use primitives_core::separator::{BASE_CLASSES, SeparatorOrientation};
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct SeparatorProps {
    /// The orientation of the separator.
    #[prop_or_default]
    pub orientation: SeparatorOrientation,

    /// Additional Tailwind classes merged onto the root element.
    #[prop_or_default]
    pub class: String,

    /// Whether the separator is purely decorative. When `true`, the separator
    /// will have `role="none"` instead of `role="separator"`.
    #[prop_or_default]
    pub decorative: bool,
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A separator component matching the shadcn/ui v4 API.
///
/// Renders an `<hr>` element styled as either a horizontal or vertical
/// divider. Uses `primitives_core::separator::SeparatorOrientation` for shared
/// class and orientation mapping logic.
///
/// ## Orientations
///
/// `Horizontal` (default) · `Vertical`
///
/// ## Example
///
/// ```rust
/// html! {
///     <Separator />
///
///     <div class="flex items-center h-10">
///         <span>{"Left"}</span>
///         <Separator orientation={SeparatorOrientation::Vertical} class="mx-4" />
///         <span>{"Right"}</span>
///     </div>
///
///     <Separator orientation={SeparatorOrientation::Horizontal} class="my-4" />
/// }
/// ```
#[component(Separator)]
pub fn separator(props: &SeparatorProps) -> Html {
    let classes = format!(
        "{} {} {}",
        BASE_CLASSES,
        props.orientation.classes(),
        props.class
    );

    let role = if props.decorative {
        "none"
    } else {
        "separator"
    };

    let data_orientation = props.orientation.as_str();

    let aria_orientation =
        if !props.decorative && props.orientation == SeparatorOrientation::Vertical {
            Some("vertical")
        } else {
            None
        };

    html! {
        <hr
            class={classes}
            role={role}
            aria-orientation={aria_orientation}
            data-slot="separator"
            data-orientation={data_orientation}
        />
    }
}

use leptos::prelude::*;
use primitives_core::separator::{BASE_CLASSES, SeparatorOrientation};

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
/// view! {
///     <Separator />
///
///     <div class="flex items-center h-10">
///         <span>"Left"</span>
///         <Separator orientation=SeparatorOrientation::Vertical class="mx-4" />
///         <span>"Right"</span>
///     </div>
///
///     <Separator orientation=SeparatorOrientation::Horizontal class="my-4" />
/// }
/// ```
#[component]
pub fn Separator(
    /// The orientation of the separator.
    #[prop(optional, into)]
    orientation: SeparatorOrientation,
    /// Additional Tailwind classes merged onto the root element.
    #[prop(optional, into)]
    class: String,
    /// Whether the separator is purely decorative. When `true`, the separator
    /// will have `role="none"` instead of `role="separator"`.
    #[prop(optional)]
    decorative: bool,
) -> impl IntoView {
    let classes = format!("{} {} {}", BASE_CLASSES, orientation.classes(), class);

    let role = if decorative { "none" } else { "separator" };
    let data_orientation = orientation.as_str();
    let aria_orientation = if !decorative && orientation == SeparatorOrientation::Vertical {
        Some("vertical")
    } else {
        None
    };

    view! {
        <hr
            class={classes}
            role={role}
            aria-orientation={aria_orientation}
            data-slot="separator"
            data-orientation={data_orientation}
        />
    }
}

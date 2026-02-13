use leptos::prelude::*;
use primitives_core::skeleton::BASE_CLASSES;

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A skeleton loading placeholder matching the shadcn/ui v4 API.
///
/// Renders a pulsing `<div>` that acts as a visual placeholder while content
/// is loading. The consumer is responsible for providing sizing and shape
/// classes (e.g. `w-48 h-4 rounded-md`) via the `class` prop.
///
/// Uses `primitives_core::skeleton::BASE_CLASSES` for the invariant Tailwind
/// animation and background classes.
///
/// ## Example
///
/// ```rust
/// view! {
///     // A rectangular text skeleton
///     <Skeleton class="h-4 w-48 rounded-md" />
///
///     // A circular avatar skeleton
///     <Skeleton class="size-10 rounded-full" />
///
///     // A card-like skeleton
///     <div class="flex flex-col gap-2">
///         <Skeleton class="h-32 w-full rounded-lg" />
///         <Skeleton class="h-4 w-3/4 rounded-md" />
///         <Skeleton class="h-4 w-1/2 rounded-md" />
///     </div>
/// }
/// ```
#[component]
pub fn Skeleton(
    /// Additional Tailwind classes merged onto the root element (e.g. width,
    /// height, border-radius). This is where you control the skeleton's
    /// dimensions and shape.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let classes = format!("{} {}", BASE_CLASSES, class);

    view! {
        <div
            class={classes}
            data-slot="skeleton"
            aria-hidden="true"
        />
    }
}

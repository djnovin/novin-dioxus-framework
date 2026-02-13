use leptos::prelude::*;
use primitives_core::loading::{CONTAINER_CLASSES, SPINNER_CLASSES};

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A loading spinner component that displays a centered spinning indicator.
///
/// Uses `primitives_core::loading::CONTAINER_CLASSES` and
/// `primitives_core::loading::SPINNER_CLASSES` for the invariant Tailwind
/// classes.
///
/// ## Example
///
/// ```rust
/// view! {
///     <Loading />
///
///     <Loading class="h-screen" spinner_class="h-12 w-12" />
/// }
/// ```
#[component]
pub fn Loading(
    /// Additional Tailwind classes merged onto the outer container element.
    #[prop(optional, into)]
    class: String,
    /// Additional Tailwind classes merged onto the spinner element (e.g. to
    /// override the default size or border color).
    #[prop(optional, into)]
    spinner_class: String,
) -> impl IntoView {
    let container_classes = format!("{} {}", CONTAINER_CLASSES, class);
    let spinner_classes = format!("{} {}", SPINNER_CLASSES, spinner_class);

    view! {
        <div
            class={container_classes}
            data-slot="loading"
            role="status"
            aria-label="Loading"
        >
            <div class={spinner_classes} />
            <span class="sr-only">"Loading…"</span>
        </div>
    }
}

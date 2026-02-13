use primitives_core::loading::{CONTAINER_CLASSES, SPINNER_CLASSES};
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct LoadingProps {
    /// Additional Tailwind classes merged onto the outer container element.
    #[prop_or_default]
    pub class: String,

    /// Additional Tailwind classes merged onto the spinner element (e.g. to
    /// override the default size or border color).
    #[prop_or_default]
    pub spinner_class: String,
}

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
/// html! {
///     <Loading />
///
///     <Loading class="h-screen" spinner_class="h-12 w-12" />
/// }
/// ```
#[component(Loading)]
pub fn loading(props: &LoadingProps) -> Html {
    let container_classes = format!("{} {}", CONTAINER_CLASSES, props.class);
    let spinner_classes = format!("{} {}", SPINNER_CLASSES, props.spinner_class);

    html! {
        <div
            class={container_classes}
            data-slot="loading"
            role="status"
            aria-label="Loading"
        >
            <div class={spinner_classes} />
            <span class="sr-only">{"Loading\u{2026}"}</span>
        </div>
    }
}

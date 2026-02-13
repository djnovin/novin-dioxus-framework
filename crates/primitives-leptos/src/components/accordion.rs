use leptos::ev;
use leptos::prelude::*;
use primitives_core::typography::TypographyVariant;

// ---------------------------------------------------------------------------
// Class constants
// ---------------------------------------------------------------------------

/// Base classes for the accordion root element.
const ACCORDION_BASE: &str = "flex flex-col gap-4 py-4 w-full group";

/// Base classes for the accordion header row.
const ACCORDION_HEADER_BASE: &str =
    "flex flex-row justify-between items-center space-x-4 cursor-pointer w-full";

/// Classes for the title text.
const ACCORDION_TITLE_CLASSES: &str = "font-light group-hover:underline";

/// Classes for the indicator (open/close icon).
const ACCORDION_INDICATOR_CLASSES: &str = "text-primary select-none";

/// Classes for the expanded content area.
const ACCORDION_CONTENT_BASE: &str = "flex flex-col gap-4";

/// Classes for the content text.
const ACCORDION_CONTENT_TEXT_CLASSES: &str = "font-light";

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// An accordion component that provides a collapsible panel with a title
/// header and expandable content area.
///
/// Supports both **controlled** (via `is_open` + `on_toggle`) and
/// **uncontrolled** (via `is_open` initial value) modes.
///
/// ## Example
///
/// ```rust
/// // Uncontrolled — starts closed, manages its own state
/// view! {
///     <Accordion
///         title="What is this?"
///         content="This is an accordion component."
///     />
/// }
///
/// // Starts open
/// view! {
///     <Accordion
///         title="FAQ Item"
///         content="Answer to the question."
///         is_open=true
///     />
/// }
///
/// // Controlled
/// view! {
///     <Accordion
///         title="Controlled"
///         content="You control the open state."
///         is_open=open.get()
///         on_toggle=move |_| set_open.update(|v| *v = !*v)
///     />
/// }
/// ```
#[component]
pub fn Accordion(
    /// The title text displayed in the accordion header.
    #[prop(into)]
    title: String,
    /// The content text displayed when the accordion is open.
    #[prop(into)]
    content: String,
    /// Whether the accordion is initially open. When `Some`, the accordion
    /// starts in the given state. Defaults to `false` (closed).
    #[prop(optional, into)]
    is_open: Option<bool>,
    /// Additional Tailwind classes merged onto the root element.
    #[prop(optional, into)]
    class: String,
    /// Callback fired when the accordion header is clicked. Receives no
    /// payload — the consumer is responsible for toggling state in controlled
    /// mode.
    #[prop(optional, into)]
    on_toggle: Option<Callback<()>>,
) -> impl IntoView {
    let (open, set_open) = signal(is_open.unwrap_or(false));

    let root_classes = format!("{} {}", ACCORDION_BASE, class);

    let title_variant_classes = TypographyVariant::H6.classes();
    let title_classes = format!("{} {}", title_variant_classes, ACCORDION_TITLE_CLASSES);

    let content_variant_classes = TypographyVariant::Body2.classes();
    let content_text_classes = format!(
        "{} {}",
        content_variant_classes, ACCORDION_CONTENT_TEXT_CLASSES
    );

    view! {
        <div
            class={root_classes}
            data-slot="accordion"
            data-state={move || if open.get() { "open" } else { "closed" }}
            on:click=move |_| {
                set_open.update(|v| *v = !*v);
                if let Some(ref handler) = on_toggle {
                    handler.run(());
                }
            }
        >
            // Header row
            <div
                class={ACCORDION_HEADER_BASE}
                data-slot="accordion-header"
                role="button"
                aria-expanded={move || open.get().to_string()}
                tabindex="0"
                on:keydown=move |evt: ev::KeyboardEvent| {
                    let key = evt.key();
                    if key == "Enter" || key == " " {
                        evt.prevent_default();
                        set_open.update(|v| *v = !*v);
                        if let Some(ref handler) = on_toggle {
                            handler.run(());
                        }
                    }
                }
            >
                <div>
                    <h6 class={title_classes.clone()}>
                        {title.clone()}
                    </h6>
                </div>
                <div class={ACCORDION_INDICATOR_CLASSES}>
                    {move || if open.get() { "\u{2212}" } else { "+" }}
                </div>
            </div>

            // Content area
            <div
                data-slot="accordion-content"
                class={move || {
                    if open.get() {
                        "block"
                    } else {
                        "hidden"
                    }
                }}
            >
                <div class={ACCORDION_CONTENT_BASE}>
                    <div>
                        <p class={content_text_classes.clone()}>
                            {content.clone()}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

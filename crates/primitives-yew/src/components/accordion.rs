use primitives_core::typography::TypographyVariant;
use yew::prelude::*;

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
// Props
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct AccordionProps {
    /// The title text displayed in the accordion header.
    pub title: String,

    /// The content text displayed when the accordion is open.
    pub content: String,

    /// Whether the accordion is initially open. When `Some`, the accordion
    /// starts in the given state. Defaults to `false` (closed).
    #[prop_or_default]
    pub is_open: Option<bool>,

    /// Additional Tailwind classes merged onto the root element.
    #[prop_or_default]
    pub class: String,

    /// Callback fired when the accordion header is clicked. Receives no
    /// payload — the consumer is responsible for toggling state in controlled
    /// mode.
    #[prop_or_default]
    pub on_toggle: Option<Callback<()>>,
}

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
/// html! {
///     <Accordion
///         title="What is this?"
///         content="This is an accordion component."
///     />
/// }
///
/// // Starts open
/// html! {
///     <Accordion
///         title="FAQ Item"
///         content="Answer to the question."
///         is_open={true}
///     />
/// }
///
/// // Controlled
/// html! {
///     <Accordion
///         title="Controlled"
///         content="You control the open state."
///         is_open={*open}
///         on_toggle={Callback::from(move |_| open.set(!*open))}
///     />
/// }
/// ```
#[component(Accordion)]
pub fn accordion(props: &AccordionProps) -> Html {
    let open = use_state(|| props.is_open.unwrap_or(false));

    let root_classes = format!("{} {}", ACCORDION_BASE, props.class);

    let title_variant_classes = TypographyVariant::H6.classes();
    let title_classes = format!("{} {}", title_variant_classes, ACCORDION_TITLE_CLASSES);

    let content_variant_classes = TypographyVariant::Body2.classes();
    let content_text_classes = format!(
        "{} {}",
        content_variant_classes, ACCORDION_CONTENT_TEXT_CLASSES
    );

    let is_open = *open;
    let data_state = if is_open { "open" } else { "closed" };

    let onclick = {
        let open = open.clone();
        let on_toggle = props.on_toggle.clone();
        Callback::from(move |_: MouseEvent| {
            open.set(!*open);
            if let Some(ref handler) = on_toggle {
                handler.emit(());
            }
        })
    };

    let onkeydown = {
        let open = open.clone();
        let on_toggle = props.on_toggle.clone();
        Callback::from(move |evt: KeyboardEvent| {
            let key = evt.key();
            if key == "Enter" || key == " " {
                evt.prevent_default();
                open.set(!*open);
                if let Some(ref handler) = on_toggle {
                    handler.emit(());
                }
            }
        })
    };

    let indicator = if is_open { "\u{2212}" } else { "+" };

    html! {
        <div
            class={root_classes}
            data-slot="accordion"
            data-state={data_state}
            onclick={onclick}
        >
            // Header row
            <div
                class={ACCORDION_HEADER_BASE}
                data-slot="accordion-header"
                role="button"
                aria-expanded={is_open.to_string()}
                tabindex="0"
                onkeydown={onkeydown}
            >
                <div>
                    <h6 class={title_classes}>
                        { &props.title }
                    </h6>
                </div>
                <div class={ACCORDION_INDICATOR_CLASSES}>
                    { indicator }
                </div>
            </div>

            // Content area
            if is_open {
                <div
                    data-slot="accordion-content"
                >
                    <div class={ACCORDION_CONTENT_BASE}>
                        <div>
                            <p class={content_text_classes}>
                                { &props.content }
                            </p>
                        </div>
                    </div>
                </div>
            } else {
                <div class="hidden" data-slot="accordion-content" />
            }
        </div>
    }
}

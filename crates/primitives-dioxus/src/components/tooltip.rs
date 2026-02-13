use dioxus::prelude::*;
use primitives_core::tooltip::{
    ARROW_BASE, CONTENT_BASE, CONTENT_HIDDEN, CONTENT_TRANSITION, CONTENT_VISIBLE, TOOLTIP_BASE,
    TOOLTIP_TRIGGER_BASE,
};

// Re-export core types so consumers don't need to depend on primitives-core
// directly.
pub use primitives_core::tooltip::{TooltipError, TooltipSide};

// ---------------------------------------------------------------------------
// Shared error rendering (Dioxus-specific)
// ---------------------------------------------------------------------------

/// Renders a visible inline error in **debug builds**, panics in **release**.
fn render_error(error: TooltipError) -> Element {
    #[cfg(debug_assertions)]
    {
        let msg = error.to_string();
        return rsx! {
            span {
                style: "color: #ef4444; background: #fef2f2; border: 1px solid #fca5a5; padding: 4px 8px; border-radius: 4px; font-size: 12px; font-family: monospace;",
                "{msg}"
            }
        };
    }

    #[cfg(not(debug_assertions))]
    {
        panic!("{error}");
    }
}

// ---------------------------------------------------------------------------
// Context types (Dioxus-specific)
// ---------------------------------------------------------------------------

/// Shared state between `Tooltip`, `TooltipTrigger`, and `TooltipContent`.
#[derive(Clone, Copy, PartialEq)]
struct TooltipContext {
    open: Signal<bool>,
}

/// Retrieves the `TooltipContext` or returns a descriptive error.
fn use_tooltip_context(component: &'static str) -> Result<TooltipContext, TooltipError> {
    try_use_context::<TooltipContext>().ok_or(TooltipError::MissingTooltipProvider { component })
}

// ---------------------------------------------------------------------------
// Tooltip (root)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// Additional Tailwind classes on the root wrapper.
    #[props(default)]
    pub class: String,
    /// Control open state externally. When `None` the component manages its
    /// own state via hover / focus on the trigger.
    #[props(default)]
    pub open: Option<bool>,
    pub children: Element,
}

/// Root wrapper for a tooltip. Provides shared open/close context consumed by
/// `TooltipTrigger` and `TooltipContent`.
///
/// ```rust
/// Tooltip {
///     TooltipTrigger {
///         Button { variant: ButtonVariant::Outline, "Hover" }
///     }
///     TooltipContent {
///         p { "Add to library" }
///     }
/// }
/// ```
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let open = use_signal(|| props.open.unwrap_or(false));
    use_context_provider(|| TooltipContext { open });

    rsx! {
        div {
            "data-slot": "tooltip",
            class: format!("{} {}", TOOLTIP_BASE, props.class),
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// TooltipTrigger
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct TooltipTriggerProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: String,
    /// The element the user hovers / focuses to open the tooltip.
    pub children: Element,
}

/// Wraps the trigger element and manages hover / focus events that toggle the
/// tooltip's open state.
#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    let ctx = match use_tooltip_context("TooltipTrigger") {
        Ok(ctx) => ctx,
        Err(e) => return render_error(e),
    };

    let mut open = ctx.open;

    rsx! {
        span {
            "data-slot": "tooltip-trigger",
            class: format!("{} {}", TOOLTIP_TRIGGER_BASE, props.class),
            onmouseenter: move |_| open.set(true),
            onmouseleave: move |_| open.set(false),
            onfocus: move |_| open.set(true),
            onblur: move |_| open.set(false),
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// TooltipContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct TooltipContentProps {
    /// Which side of the trigger the content appears on.
    #[props(default)]
    pub side: TooltipSide,
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: String,
    /// Whether to render the small arrow pointing at the trigger.
    #[props(default = true)]
    pub show_arrow: bool,
    /// Tooltip body (text, icons, etc.).
    pub children: Element,
}

/// The floating content of a tooltip. Always present in the DOM so that CSS
/// transitions work in both directions; visibility is toggled via opacity and
/// pointer-events.
///
/// ```rust
/// TooltipContent {
///     side: TooltipSide::Bottom,
///     p { "This appears below the trigger" }
/// }
/// ```
#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    let ctx = match use_tooltip_context("TooltipContent") {
        Ok(ctx) => ctx,
        Err(e) => return render_error(e),
    };

    let is_open = *ctx.open.read();

    let visibility_classes = if is_open {
        CONTENT_VISIBLE
    } else {
        CONTENT_HIDDEN
    };

    let state = if is_open { "open" } else { "closed" };

    rsx! {
        div {
            "data-slot": "tooltip-content",
            "data-state": state,
            "data-side": props.side.as_str(),
            role: "tooltip",
            class: format!(
                "absolute {} {} {} {} {}",
                props.side.position_classes(),
                CONTENT_BASE,
                CONTENT_TRANSITION,
                visibility_classes,
                props.class,
            ),

            {props.children}

            if props.show_arrow {
                // Small rotated square that acts as the arrow / caret.
                div {
                    "data-slot": "tooltip-arrow",
                    class: format!(
                        "{} {}",
                        ARROW_BASE,
                        props.side.arrow_classes(),
                    ),
                }
            }
        }
    }
}

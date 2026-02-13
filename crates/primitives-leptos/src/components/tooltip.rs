use leptos::prelude::*;
use primitives_core::tooltip::{
    ARROW_BASE, CONTENT_BASE, CONTENT_HIDDEN, CONTENT_TRANSITION, CONTENT_VISIBLE, TOOLTIP_BASE,
    TOOLTIP_TRIGGER_BASE, TooltipError, TooltipSide,
};

// ---------------------------------------------------------------------------
// Error rendering (Leptos-specific)
// ---------------------------------------------------------------------------

fn render_error(error: TooltipError) -> impl IntoView {
    #[cfg(debug_assertions)]
    {
        let msg = error.to_string();
        view! {
            <span style="color: #ef4444; background: #fef2f2; border: 1px solid #fca5a5; padding: 4px 8px; border-radius: 4px; font-size: 12px; font-family: monospace;">
                {msg}
            </span>
        }
        .into_any()
    }

    #[cfg(not(debug_assertions))]
    {
        panic!("{error}");
    }
}

// ---------------------------------------------------------------------------
// Context types (Leptos-specific)
// ---------------------------------------------------------------------------

/// Shared state between `Tooltip`, `TooltipTrigger`, and `TooltipContent`.
#[derive(Clone, Copy)]
struct TooltipContext {
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
}

// ---------------------------------------------------------------------------
// Tooltip (root)
// ---------------------------------------------------------------------------

/// Root wrapper for a tooltip. Provides shared open/close context consumed by
/// `TooltipTrigger` and `TooltipContent`.
///
/// ```rust
/// view! {
///     <Tooltip>
///         <TooltipTrigger>
///             <Button variant=ButtonVariant::Outline>"Hover"</Button>
///         </TooltipTrigger>
///         <TooltipContent>
///             <p>"Add to library"</p>
///         </TooltipContent>
///     </Tooltip>
/// }
/// ```
#[component]
pub fn Tooltip(
    /// Additional Tailwind classes on the root wrapper.
    #[prop(optional, into)]
    class: String,
    /// Control open state externally. When `None` the component manages its
    /// own state via hover / focus on the trigger.
    #[prop(optional, into)]
    open: Option<bool>,
    children: Children,
) -> impl IntoView {
    let (open_signal, set_open_signal) = signal(open.unwrap_or(false));
    provide_context(TooltipContext {
        open: open_signal,
        set_open: set_open_signal,
    });

    let classes = format!("{} {}", TOOLTIP_BASE, class);

    view! {
        <div
            data-slot="tooltip"
            class={classes}
        >
            {children()}
        </div>
    }
}

// ---------------------------------------------------------------------------
// TooltipTrigger
// ---------------------------------------------------------------------------

/// Wraps the trigger element and manages hover / focus events that toggle the
/// tooltip's open state.
#[component]
pub fn TooltipTrigger(
    /// Additional Tailwind classes.
    #[prop(optional, into)]
    class: String,
    /// The element the user hovers / focuses to open the tooltip.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<TooltipContext>();

    match ctx {
        None => render_error(TooltipError::MissingTooltipProvider {
            component: "TooltipTrigger",
        })
        .into_any(),
        Some(ctx) => {
            let set_open = ctx.set_open;
            let classes = format!("{} {}", TOOLTIP_TRIGGER_BASE, class);

            view! {
                <span
                    data-slot="tooltip-trigger"
                    class={classes}
                    on:mouseenter=move |_| set_open.set(true)
                    on:mouseleave=move |_| set_open.set(false)
                    on:focus=move |_| set_open.set(true)
                    on:blur=move |_| set_open.set(false)
                >
                    {children()}
                </span>
            }
            .into_any()
        }
    }
}

// ---------------------------------------------------------------------------
// TooltipContent
// ---------------------------------------------------------------------------

/// The floating content of a tooltip. Always present in the DOM so that CSS
/// transitions work in both directions; visibility is toggled via opacity and
/// pointer-events.
///
/// ```rust
/// view! {
///     <TooltipContent side=TooltipSide::Bottom>
///         <p>"This appears below the trigger"</p>
///     </TooltipContent>
/// }
/// ```
#[component]
pub fn TooltipContent(
    /// Which side of the trigger the content appears on.
    #[prop(optional, into)]
    side: TooltipSide,
    /// Additional Tailwind classes.
    #[prop(optional, into)]
    class: String,
    /// Whether to render the small arrow pointing at the trigger.
    #[prop(optional, default = true)]
    show_arrow: bool,
    /// Tooltip body (text, icons, etc.).
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<TooltipContext>();

    match ctx {
        None => render_error(TooltipError::MissingTooltipProvider {
            component: "TooltipContent",
        })
        .into_any(),
        Some(ctx) => {
            let open = ctx.open;

            let data_side = side.as_str();
            let position = side.position_classes();
            let arrow_position = side.arrow_classes();

            view! {
                <div
                    data-slot="tooltip-content"
                    data-state={move || if open.get() { "open" } else { "closed" }}
                    data-side={data_side}
                    role="tooltip"
                    class={move || {
                        let visibility = if open.get() {
                            CONTENT_VISIBLE
                        } else {
                            CONTENT_HIDDEN
                        };
                        format!(
                            "absolute {} {} {} {} {}",
                            position,
                            CONTENT_BASE,
                            CONTENT_TRANSITION,
                            visibility,
                            class,
                        )
                    }}
                >
                    {children()}

                    {if show_arrow {
                        Some(view! {
                            <div
                                data-slot="tooltip-arrow"
                                class={format!("{} {}", ARROW_BASE, arrow_position)}
                            />
                        })
                    } else {
                        None
                    }}
                </div>
            }
            .into_any()
        }
    }
}

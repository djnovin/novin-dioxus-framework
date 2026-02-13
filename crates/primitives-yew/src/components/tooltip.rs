use primitives_core::tooltip::{
    ARROW_BASE, CONTENT_BASE, CONTENT_HIDDEN, CONTENT_TRANSITION, CONTENT_VISIBLE, TOOLTIP_BASE,
    TOOLTIP_TRIGGER_BASE, TooltipError, TooltipSide,
};
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Context types (Yew-specific)
// ---------------------------------------------------------------------------

/// Shared state between `Tooltip`, `TooltipTrigger`, and `TooltipContent`.
#[derive(Clone, PartialEq)]
pub struct TooltipContext {
    pub open: UseStateHandle<bool>,
}

// ---------------------------------------------------------------------------
// Error rendering (Yew-specific)
// ---------------------------------------------------------------------------

fn render_error(error: TooltipError) -> Html {
    #[cfg(debug_assertions)]
    {
        let msg = error.to_string();
        html! {
            <span style="color: #ef4444; background: #fef2f2; border: 1px solid #fca5a5; padding: 4px 8px; border-radius: 4px; font-size: 12px; font-family: monospace;">
                {msg}
            </span>
        }
    }

    #[cfg(not(debug_assertions))]
    {
        panic!("{error}");
    }
}

// ---------------------------------------------------------------------------
// Tooltip (root)
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct TooltipProps {
    /// Additional Tailwind classes on the root wrapper.
    #[prop_or_default]
    pub class: String,

    /// Control open state externally. When `None` the component manages its
    /// own state via hover / focus on the trigger.
    #[prop_or_default]
    pub open: Option<bool>,

    pub children: Children,
}

/// Root wrapper for a tooltip. Provides shared open/close context consumed by
/// `TooltipTrigger` and `TooltipContent`.
///
/// ```rust
/// html! {
///     <Tooltip>
///         <TooltipTrigger>
///             <Button variant={ButtonVariant::Outline}>{"Hover"}</Button>
///         </TooltipTrigger>
///         <TooltipContent>
///             <p>{"Add to library"}</p>
///         </TooltipContent>
///     </Tooltip>
/// }
/// ```
#[component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    let open = use_state(|| props.open.unwrap_or(false));
    let ctx = TooltipContext { open: open.clone() };

    let classes = format!("{} {}", TOOLTIP_BASE, props.class);

    html! {
        <ContextProvider<TooltipContext> context={ctx}>
            <div
                data-slot="tooltip"
                class={classes}
            >
                { for props.children.iter() }
            </div>
        </ContextProvider<TooltipContext>>
    }
}

// ---------------------------------------------------------------------------
// TooltipTrigger
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct TooltipTriggerProps {
    /// Additional Tailwind classes.
    #[prop_or_default]
    pub class: String,

    /// The element the user hovers / focuses to open the tooltip.
    pub children: Children,
}

/// Wraps the trigger element and manages hover / focus events that toggle the
/// tooltip's open state.
#[component(TooltipTrigger)]
pub fn tooltip_trigger(props: &TooltipTriggerProps) -> Html {
    let ctx = use_context::<TooltipContext>();

    match ctx {
        None => render_error(TooltipError::MissingTooltipProvider {
            component: "TooltipTrigger",
        }),
        Some(ctx) => {
            let classes = format!("{} {}", TOOLTIP_TRIGGER_BASE, props.class);

            let onmouseenter = {
                let open = ctx.open.clone();
                Callback::from(move |_: MouseEvent| {
                    open.set(true);
                })
            };

            let onmouseleave = {
                let open = ctx.open.clone();
                Callback::from(move |_: MouseEvent| {
                    open.set(false);
                })
            };

            let onfocus = {
                let open = ctx.open.clone();
                Callback::from(move |_: FocusEvent| {
                    open.set(true);
                })
            };

            let onblur = {
                let open = ctx.open.clone();
                Callback::from(move |_: FocusEvent| {
                    open.set(false);
                })
            };

            html! {
                <span
                    data-slot="tooltip-trigger"
                    class={classes}
                    onmouseenter={onmouseenter}
                    onmouseleave={onmouseleave}
                    onfocus={onfocus}
                    onblur={onblur}
                >
                    { for props.children.iter() }
                </span>
            }
        }
    }
}

// ---------------------------------------------------------------------------
// TooltipContent
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct TooltipContentProps {
    /// Which side of the trigger the content appears on.
    #[prop_or_default]
    pub side: TooltipSide,

    /// Additional Tailwind classes.
    #[prop_or_default]
    pub class: String,

    /// Whether to render the small arrow pointing at the trigger.
    #[prop_or(true)]
    pub show_arrow: bool,

    /// Tooltip body (text, icons, etc.).
    pub children: Children,
}

/// The floating content of a tooltip. Always present in the DOM so that CSS
/// transitions work in both directions; visibility is toggled via opacity and
/// pointer-events.
///
/// ```rust
/// html! {
///     <TooltipContent side={TooltipSide::Bottom}>
///         <p>{"This appears below the trigger"}</p>
///     </TooltipContent>
/// }
/// ```
#[component(TooltipContent)]
pub fn tooltip_content(props: &TooltipContentProps) -> Html {
    let ctx = use_context::<TooltipContext>();

    match ctx {
        None => render_error(TooltipError::MissingTooltipProvider {
            component: "TooltipContent",
        }),
        Some(ctx) => {
            let is_open = *ctx.open;

            let visibility_classes = if is_open {
                CONTENT_VISIBLE
            } else {
                CONTENT_HIDDEN
            };

            let state = if is_open { "open" } else { "closed" };

            let data_side = props.side.as_str();
            let position = props.side.position_classes();
            let arrow_position = props.side.arrow_classes();

            let classes = format!(
                "absolute {} {} {} {} {}",
                position, CONTENT_BASE, CONTENT_TRANSITION, visibility_classes, props.class,
            );

            let arrow_classes = format!("{} {}", ARROW_BASE, arrow_position);

            html! {
                <div
                    data-slot="tooltip-content"
                    data-state={state}
                    data-side={data_side}
                    role="tooltip"
                    class={classes}
                >
                    { for props.children.iter() }

                    if props.show_arrow {
                        <div
                            data-slot="tooltip-arrow"
                            class={arrow_classes}
                        />
                    }
                </div>
            }
        }
    }
}

use thiserror::Error;

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum TooltipError {
    #[error("<{component}> must be used within a <Tooltip> component.")]
    MissingTooltipProvider { component: &'static str },
}

// ---------------------------------------------------------------------------
// TooltipSide enum
// ---------------------------------------------------------------------------

/// Which side of the trigger the tooltip content should appear on.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum TooltipSide {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl TooltipSide {
    /// Kebab-case string for the `data-side` attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    /// Tailwind positioning classes that place the content on this side of the
    /// trigger. The trigger's parent (`Tooltip`) is `relative`, so these
    /// absolute offsets work against the trigger's bounding box.
    pub fn position_classes(&self) -> &'static str {
        match self {
            Self::Top => "bottom-full left-1/2 -translate-x-1/2 mb-2",
            Self::Bottom => "top-full left-1/2 -translate-x-1/2 mt-2",
            Self::Left => "right-full top-1/2 -translate-y-1/2 mr-2",
            Self::Right => "left-full top-1/2 -translate-y-1/2 ml-2",
        }
    }

    /// Tailwind classes that position the small arrow element so it points back
    /// at the trigger.
    pub fn arrow_classes(&self) -> &'static str {
        match self {
            Self::Top => "left-1/2 -translate-x-1/2 top-full -mt-[3px]",
            Self::Bottom => "left-1/2 -translate-x-1/2 bottom-full -mb-[3px]",
            Self::Left => "top-1/2 -translate-y-1/2 left-full -ml-[3px]",
            Self::Right => "top-1/2 -translate-y-1/2 right-full -mr-[3px]",
        }
    }
}

// ---------------------------------------------------------------------------
// Class constants
// ---------------------------------------------------------------------------

/// Base classes for the `Tooltip` root wrapper.
pub const TOOLTIP_BASE: &str = "relative inline-flex";

/// Base classes for the `TooltipTrigger` wrapper.
pub const TOOLTIP_TRIGGER_BASE: &str = "inline-flex";

/// Base visual classes for the content bubble — matches the shadcn v4 style.
pub const CONTENT_BASE: &str = "\
    z-50 w-fit rounded-md px-3 py-1.5 text-xs text-balance \
    bg-foreground text-background";

/// Transition classes applied to both open and closed states.
pub const CONTENT_TRANSITION: &str = "transition-all duration-150 ease-out";

/// Visibility classes when the tooltip is **open**.
pub const CONTENT_VISIBLE: &str = "pointer-events-auto opacity-100 scale-100";

/// Visibility classes when the tooltip is **closed**.
pub const CONTENT_HIDDEN: &str = "pointer-events-none opacity-0 scale-95";

/// Base classes for the tooltip arrow (a small rotated square).
pub const ARROW_BASE: &str = "absolute size-2 rotate-45 rounded-[1px] bg-foreground";

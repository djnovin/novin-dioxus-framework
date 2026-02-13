// ---------------------------------------------------------------------------
// Variant enum
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

impl ToggleVariant {
    /// Kebab-case string for the `data-variant` attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Outline => "outline",
        }
    }

    /// Tailwind classes for this variant — taken verbatim from the shadcn v4
    /// `toggle.tsx` source.
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Default => "bg-transparent",
            Self::Outline => {
                "border border-input bg-transparent shadow-xs hover:bg-accent hover:text-accent-foreground"
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Size enum
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ToggleSize {
    Sm,
    #[default]
    Default,
    Lg,
}

impl ToggleSize {
    /// Kebab-case string for the `data-size` attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Default => "default",
            Self::Lg => "lg",
        }
    }

    /// Tailwind classes for this size — taken verbatim from the shadcn v4
    /// `toggle.tsx` source.
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Sm => "h-8 px-1.5 min-w-8",
            Self::Default => "h-9 px-2 min-w-9",
            Self::Lg => "h-10 px-2.5 min-w-10",
        }
    }
}

// ---------------------------------------------------------------------------
// Base classes (shared across every variant / size)
// ---------------------------------------------------------------------------

/// The invariant Tailwind classes applied to every toggle, matching shadcn v4's
/// `cva` base string.
pub const BASE_CLASSES: &str = "\
    inline-flex items-center justify-center gap-2 rounded-md \
    text-sm font-medium whitespace-nowrap \
    hover:bg-muted hover:text-muted-foreground \
    disabled:pointer-events-none disabled:opacity-50 \
    data-[state=on]:bg-accent data-[state=on]:text-accent-foreground \
    [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0 \
    outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] \
    transition-[color,box-shadow] \
    aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 \
    aria-invalid:border-destructive";

// ---------------------------------------------------------------------------
// Variant enum
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl ButtonVariant {
    /// Returns the kebab-case string used for the `data-variant` attribute,
    /// matching shadcn's convention.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
            Self::Outline => "outline",
            Self::Secondary => "secondary",
            Self::Ghost => "ghost",
            Self::Link => "link",
        }
    }

    /// Tailwind classes for this variant — taken verbatim from the shadcn v4
    /// `button.tsx` source.
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            Self::Destructive => {
                "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60"
            }
            Self::Outline => {
                "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50"
            }
            Self::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            Self::Ghost => "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
            Self::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

// ---------------------------------------------------------------------------
// Size enum
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Icon,
    IconXs,
    IconSm,
    IconLg,
}

impl ButtonSize {
    /// Returns the kebab-case string used for the `data-size` attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::Lg => "lg",
            Self::Icon => "icon",
            Self::IconXs => "icon-xs",
            Self::IconSm => "icon-sm",
            Self::IconLg => "icon-lg",
        }
    }

    /// Tailwind classes for this size — taken verbatim from the shadcn v4
    /// `button.tsx` source.
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Default => "h-9 px-4 py-2 has-[>svg]:px-3",
            Self::Xs => {
                "h-6 gap-1 rounded-md px-2 text-xs has-[>svg]:px-1.5 [&_svg:not([class*='size-'])]:size-3"
            }
            Self::Sm => "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5",
            Self::Lg => "h-10 rounded-md px-6 has-[>svg]:px-4",
            Self::Icon => "size-9",
            Self::IconXs => "size-6 rounded-md [&_svg:not([class*='size-'])]:size-3",
            Self::IconSm => "size-8",
            Self::IconLg => "size-10",
        }
    }
}

// ---------------------------------------------------------------------------
// Base classes (shared across every variant / size)
// ---------------------------------------------------------------------------

/// The invariant Tailwind classes applied to every button, matching shadcn v4's
/// `cva` base string.
pub const BASE_CLASSES: &str = "\
    inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md \
    text-sm font-medium transition-all \
    disabled:pointer-events-none disabled:opacity-50 \
    [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 \
    shrink-0 [&_svg]:shrink-0 \
    outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] \
    aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive";

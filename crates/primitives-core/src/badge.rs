// ---------------------------------------------------------------------------
// Variant enum
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Destructive,
}

impl BadgeVariant {
    /// Kebab-case string for the `data-variant` attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Secondary => "secondary",
            Self::Outline => "outline",
            Self::Destructive => "destructive",
        }
    }

    /// Tailwind classes for this variant.
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Default => {
                "border-transparent bg-primary text-primary-foreground shadow hover:bg-primary/80"
            }
            Self::Secondary => {
                "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }
            Self::Outline => "text-foreground",
            Self::Destructive => {
                "border-transparent bg-destructive text-destructive-foreground shadow hover:bg-destructive/80"
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Base classes
// ---------------------------------------------------------------------------

/// The invariant Tailwind classes applied to every badge.
pub const BASE_CLASSES: &str = "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2";

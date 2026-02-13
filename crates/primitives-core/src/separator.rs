// ---------------------------------------------------------------------------
// Orientation enum
// ---------------------------------------------------------------------------

/// The orientation of a separator.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl SeparatorOrientation {
    /// Kebab-case string for the `data-orientation` attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    /// Tailwind classes for this orientation.
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Horizontal => "w-full",
            Self::Vertical => "h-full",
        }
    }
}

// ---------------------------------------------------------------------------
// Base classes
// ---------------------------------------------------------------------------

/// The invariant Tailwind classes applied to every separator.
pub const BASE_CLASSES: &str = "border-t border-gray-200";

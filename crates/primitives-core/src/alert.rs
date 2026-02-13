// ---------------------------------------------------------------------------
// Variant enum
// ---------------------------------------------------------------------------

/// All supported alert variants.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl AlertVariant {
    /// Kebab-case string for the `data-variant` attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Tailwind classes for this variant.
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Info => "bg-gray-50 border-gray-200 border text-black",
            Self::Success => "bg-green-100 border-green-200 text-green-800",
            Self::Warning => "bg-yellow-100 border-yellow-200 text-yellow-800",
            Self::Error => "bg-red-100 border-red-200 text-red-800",
        }
    }
}

// ---------------------------------------------------------------------------
// Base classes
// ---------------------------------------------------------------------------

/// The invariant Tailwind classes applied to every alert.
pub const BASE_CLASSES: &str =
    "p-4 bg-white rounded-sm border border-gray-200 shadow-sm text-black w-full";

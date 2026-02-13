// ---------------------------------------------------------------------------
// Section type enum
// ---------------------------------------------------------------------------

/// The layout type of a section.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum SectionType {
    #[default]
    Full,
    Grid,
}

impl SectionType {
    /// Kebab-case string for the `data-type` attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Grid => "grid",
        }
    }

    /// Tailwind classes for this section type.
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Full => "w-full h-screen",
            Self::Grid => "grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 px-4",
        }
    }
}

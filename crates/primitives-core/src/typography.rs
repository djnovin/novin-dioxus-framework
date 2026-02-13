// ---------------------------------------------------------------------------
// Variant enum
// ---------------------------------------------------------------------------

/// All supported typography variants. Each variant maps to a semantic HTML tag
/// and a set of Tailwind classes.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum TypographyVariant {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Subtitle1,
    Subtitle2,
    #[default]
    Body1,
    Body2,
    Caption,
    Overline,
    Blockquote,
    Muted,
}

impl TypographyVariant {
    /// Tailwind classes for this variant.
    pub fn classes(&self) -> &'static str {
        match self {
            Self::H1 => "text-6xl",
            Self::H2 => "text-3xl",
            Self::H3 => "text-2xl",
            Self::H4 => "text-xl",
            Self::H5 => "text-lg",
            Self::H6 => "text-base",
            Self::Subtitle1 => "text-lg font-medium uppercase",
            Self::Subtitle2 => "text-base font-medium uppercase",
            Self::Body1 => "text-lg",
            Self::Body2 => "text-base",
            Self::Caption => "text-sm",
            Self::Overline => "text-xs uppercase tracking-widest font-medium",
            Self::Blockquote => "mt-6 border-s-2 ps-6 italic",
            Self::Muted => "text-sm text-muted-foreground",
        }
    }

    /// The semantic HTML tag name that should be used when rendering this
    /// variant. Framework crates use this to decide which element to emit.
    pub fn semantic_tag(&self) -> &'static str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 | Self::Subtitle1 | Self::Subtitle2 => "h6",
            Self::Body1 | Self::Body2 | Self::Caption | Self::Overline | Self::Muted => "p",
            Self::Blockquote => "blockquote",
        }
    }
}

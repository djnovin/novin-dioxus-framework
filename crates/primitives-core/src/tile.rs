// ---------------------------------------------------------------------------
// Kind enum
// ---------------------------------------------------------------------------

/// The visual kind of a tile.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum TileKind {
    #[default]
    Selection,
    Action,
}

impl TileKind {
    /// Kebab-case string for the `data-kind` attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Selection => "selection",
            Self::Action => "action",
        }
    }

    /// Tailwind classes for this kind.
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Selection => "bg-gray-50",
            Self::Action => "bg-white",
        }
    }
}

// ---------------------------------------------------------------------------
// Alignment enum
// ---------------------------------------------------------------------------

/// Logical alignment for tile header / body sections. Uses `justify-start` and
/// `justify-end` (logical properties) so alignment flips correctly in RTL.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum TileAlignment {
    #[default]
    Start,
    Center,
    End,
}

impl TileAlignment {
    /// Kebab-case string for the `data-align` attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }

    /// Tailwind justify-content class for this alignment.
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Start => "justify-start",
            Self::Center => "justify-center",
            Self::End => "justify-end",
        }
    }
}

// ---------------------------------------------------------------------------
// Class constants
// ---------------------------------------------------------------------------

/// Base classes for the tile root element.
pub const TILE_BASE: &str = "relative";

/// Classes for the tile header row. Includes `rtl:space-x-reverse` for RTL
/// support.
pub const TILE_HEADER_BASE: &str = "flex items-center space-x-2 rtl:space-x-reverse p-2";

/// Classes for the tile body row. Includes `rtl:space-x-reverse` for RTL
/// support.
pub const TILE_BODY_BASE: &str = "flex items-center space-x-2 rtl:space-x-reverse p-2";

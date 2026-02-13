//! # primitives-core
//!
//! Framework-agnostic core types and Tailwind class logic for the UI component
//! library. This crate contains **zero** framework dependencies — only enums,
//! class constants, error types, and helper methods.
//!
//! Framework-specific rendering crates (e.g. `primitives` for Dioxus,
//! `primitives-leptos`, `primitives-yew`) depend on this crate and re-export
//! its types alongside their own component implementations.

pub mod alert;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod card;
pub mod input;
pub mod link;
pub mod loading;
pub mod section;
pub mod separator;
pub mod skeleton;
pub mod text_area;
pub mod tile;
pub mod toggle;
pub mod tooltip;
pub mod typography;

/// Convenience prelude that re-exports the most commonly used types so
/// framework crates (and end users) can do:
///
/// ```rust
/// use primitives_core::prelude::*;
/// ```
pub mod prelude {
    // Avatar
    pub use crate::avatar::{AvatarError, ImageLoadingStatus};

    // Badge
    pub use crate::badge::BadgeVariant;

    // Button
    pub use crate::button::{ButtonSize, ButtonVariant};

    // Input (no types — only the BASE_CLASSES constant)

    // Toggle
    pub use crate::toggle::{ToggleSize, ToggleVariant};

    // Tooltip
    pub use crate::tooltip::{TooltipError, TooltipSide};

    // Typography
    pub use crate::typography::TypographyVariant;

    // Alert
    pub use crate::alert::AlertVariant;

    // Separator
    pub use crate::separator::SeparatorOrientation;

    // Tile
    pub use crate::tile::{TileAlignment, TileKind};

    // Section
    pub use crate::section::SectionType;
}

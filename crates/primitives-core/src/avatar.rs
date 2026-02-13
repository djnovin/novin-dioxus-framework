use thiserror::Error;

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum AvatarError {
    #[error("<{component}> must be used within an <Avatar> component.")]
    MissingAvatarProvider { component: &'static str },
}

// ---------------------------------------------------------------------------
// Image loading status (shared between AvatarImage and AvatarFallback via
// framework-specific context)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ImageLoadingStatus {
    Loading,
    Loaded,
    Error,
}

// ---------------------------------------------------------------------------
// Class constants
// ---------------------------------------------------------------------------

/// Base classes for the `Avatar` root element.
pub const AVATAR_BASE: &str = "relative flex shrink-0 overflow-hidden rounded-full size-10";

/// Extra classes added to an `Avatar` when it is nested inside an
/// `AvatarGroup` (the ring creates visible separation between stacked
/// avatars).
pub const AVATAR_GROUP_RING: &str = "ring-2 ring-background";

/// Classes for the `AvatarImage` `<img>` element.
pub const AVATAR_IMAGE_BASE: &str = "aspect-square size-full object-cover";

/// Classes for the `AvatarFallback` overlay. Uses `absolute inset-0` so it
/// sits on top of the (potentially blank) `<img>` during loading / error.
pub const AVATAR_FALLBACK_BASE: &str =
    "absolute inset-0 flex items-center justify-center rounded-full bg-muted text-sm font-medium";

/// Classes for the `AvatarBadge` status indicator. Uses logical `end-0`
/// instead of physical `right-0` for RTL support.
pub const AVATAR_BADGE_BASE: &str =
    "absolute bottom-0 end-0 block size-3 rounded-full ring-2 ring-background";

/// Classes for the `AvatarGroup` flex container. Includes
/// `rtl:space-x-reverse` for RTL support.
pub const AVATAR_GROUP_BASE: &str = "flex -space-x-3 rtl:space-x-reverse";

/// Classes for the `AvatarGroupCount` pill.
pub const AVATAR_GROUP_COUNT_BASE: &str = "relative flex shrink-0 items-center justify-center rounded-full bg-muted text-xs font-medium size-10 ring-2 ring-background";

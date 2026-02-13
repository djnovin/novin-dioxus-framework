use dioxus::prelude::*;
use primitives_core::avatar::{
    AVATAR_BADGE_BASE, AVATAR_BASE, AVATAR_FALLBACK_BASE, AVATAR_GROUP_BASE,
    AVATAR_GROUP_COUNT_BASE, AVATAR_GROUP_RING, AVATAR_IMAGE_BASE,
};

// Re-export core types so consumers don't need to depend on primitives-core
// directly.
pub use primitives_core::avatar::{AvatarError, ImageLoadingStatus};

// ---------------------------------------------------------------------------
// Shared error rendering (Dioxus-specific)
// ---------------------------------------------------------------------------

/// Renders a visible inline error message in **debug builds** so the developer
/// can immediately see what went wrong without checking the console. In
/// **release builds** the error is promoted to a panic.
fn render_error(error: AvatarError) -> Element {
    #[cfg(debug_assertions)]
    {
        let msg = error.to_string();
        return rsx! {
            span {
                style: "color: #ef4444; background: #fef2f2; border: 1px solid #fca5a5; padding: 4px 8px; border-radius: 4px; font-size: 12px; font-family: monospace;",
                "{msg}"
            }
        };
    }

    #[cfg(not(debug_assertions))]
    {
        panic!("{error}");
    }
}

// ---------------------------------------------------------------------------
// Context types (Dioxus-specific — wraps core's ImageLoadingStatus in a signal)
// ---------------------------------------------------------------------------

/// Context provided by `Avatar` to all child components. Wrapping the signal in
/// a named struct avoids accidental collisions with unrelated contexts
/// elsewhere in the tree.
#[derive(Clone, Copy, PartialEq)]
struct AvatarContext {
    status: Signal<ImageLoadingStatus>,
}

/// Marker context provided by `AvatarGroup` so that child `Avatar` components
/// can automatically add ring/border styling for the overlapping group look.
#[derive(Clone, Copy, PartialEq, Debug)]
struct AvatarGroupCtx;

/// Retrieves the `AvatarContext` from the component tree, returning a
/// descriptive `AvatarError` if the context is missing.
fn use_avatar_context(component: &'static str) -> Result<AvatarContext, AvatarError> {
    try_use_context::<AvatarContext>().ok_or(AvatarError::MissingAvatarProvider { component })
}

// ---------------------------------------------------------------------------
// Avatar (root)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Additional Tailwind classes to merge onto the root element.
    #[props(default)]
    pub class: String,
    pub children: Element,
}

/// Root wrapper for an avatar. Provides image-loading context to children.
///
/// ```rust
/// Avatar {
///     AvatarImage { src: "https://example.com/photo.png", alt: "User" }
///     AvatarFallback { "UN" }
/// }
/// ```
#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let status = use_signal(|| ImageLoadingStatus::Loading);
    use_context_provider(|| AvatarContext { status });

    let is_grouped = try_use_context::<AvatarGroupCtx>().is_some();
    let group_class = if is_grouped { AVATAR_GROUP_RING } else { "" };

    rsx! {
        span {
            class: format!("{} {} {}", AVATAR_BASE, group_class, props.class),
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarImage
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AvatarImageProps {
    /// Image source URL.
    pub src: String,
    /// Accessible alt text.
    #[props(default)]
    pub alt: String,
    /// Additional Tailwind classes for the `<img>` element.
    #[props(default)]
    pub class: String,
}

/// The `<img>` inside an `Avatar`. Communicates load / error status to the
/// parent context so that `AvatarFallback` can react accordingly.
#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    let ctx = match use_avatar_context("AvatarImage") {
        Ok(ctx) => ctx,
        Err(e) => return render_error(e),
    };

    let mut status = ctx.status;

    rsx! {
        img {
            class: format!("{} {}", AVATAR_IMAGE_BASE, props.class),
            src: "{props.src}",
            alt: "{props.alt}",
            onload: move |_| status.set(ImageLoadingStatus::Loaded),
            onerror: move |_| status.set(ImageLoadingStatus::Error),
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarFallback
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AvatarFallbackProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: String,
    /// Typically one or two letters (initials) or an icon.
    pub children: Element,
}

/// Fallback content rendered when the avatar image is still loading or failed.
/// Automatically hides once the image has loaded successfully.
#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    let ctx = match use_avatar_context("AvatarFallback") {
        Ok(ctx) => ctx,
        Err(e) => return render_error(e),
    };

    let status = ctx.status;

    if *status.read() == ImageLoadingStatus::Loaded {
        return rsx! {};
    }

    rsx! {
        span {
            class: format!("{} {}", AVATAR_FALLBACK_BASE, props.class),
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarBadge
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AvatarBadgeProps {
    /// Additional Tailwind classes (e.g. `bg-green-600` for an "online" dot).
    #[props(default)]
    pub class: String,
    /// Optional children (e.g. an icon inside the badge).
    #[props(default)]
    pub children: Element,
}

/// A small status indicator badge positioned at the bottom-end of an Avatar.
///
/// ```rust
/// Avatar {
///     AvatarImage { src: "…", alt: "…" }
///     AvatarFallback { "CN" }
///     AvatarBadge { class: "bg-green-600" }
/// }
/// ```
#[component]
pub fn AvatarBadge(props: AvatarBadgeProps) -> Element {
    if let Err(e) = use_avatar_context("AvatarBadge") {
        return render_error(e);
    }

    rsx! {
        span {
            class: format!("{} {}", AVATAR_BADGE_BASE, props.class),
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarGroup
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AvatarGroupProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: String,
    /// The `Avatar` (and optional `AvatarGroupCount`) children.
    pub children: Element,
}

/// Renders a row of overlapping avatars. Child `Avatar` components
/// automatically receive ring styling when nested inside a group.
///
/// ```rust
/// AvatarGroup {
///     Avatar { AvatarImage { src: "…", alt: "…" } AvatarFallback { "A" } }
///     Avatar { AvatarImage { src: "…", alt: "…" } AvatarFallback { "B" } }
///     AvatarGroupCount { "+3" }
/// }
/// ```
#[component]
pub fn AvatarGroup(props: AvatarGroupProps) -> Element {
    use_context_provider(|| AvatarGroupCtx);

    rsx! {
        div {
            class: format!("{} {}", AVATAR_GROUP_BASE, props.class),
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarGroupCount
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AvatarGroupCountProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: String,
    /// The count label, e.g. `"+3"`.
    pub children: Element,
}

/// A pill / circle that shows the number of remaining avatars not displayed in
/// the group.
#[component]
pub fn AvatarGroupCount(props: AvatarGroupCountProps) -> Element {
    rsx! {
        span {
            class: format!("{} {}", AVATAR_GROUP_COUNT_BASE, props.class),
            {props.children}
        }
    }
}

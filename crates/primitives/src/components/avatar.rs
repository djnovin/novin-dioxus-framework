use dioxus::prelude::*;
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
// Context types (shared between sub-components via Dioxus context)
// ---------------------------------------------------------------------------

/// Tracks the loading state of the avatar image so that `AvatarFallback` knows
/// whether it should render.
#[derive(Clone, Copy, PartialEq, Debug)]
enum ImageLoadingStatus {
    Loading,
    Loaded,
    Error,
}

/// Context provided by `Avatar` to all child components. Wrapping the signal in
/// a named struct avoids accidental collisions with unrelated
/// `Signal<ImageLoadingStatus>` contexts elsewhere in the tree.
#[derive(Clone, Copy, PartialEq)]
struct AvatarContext {
    status: Signal<ImageLoadingStatus>,
}

/// Marker context provided by `AvatarGroup` so that child `Avatar` components
/// can automatically add ring/border styling for the overlapping group look.
#[derive(Clone, Copy, PartialEq, Debug)]
struct AvatarGroupCtx;

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Retrieves the `AvatarContext` from the component tree, returning a
/// descriptive `AvatarError` if the context is missing.
fn use_avatar_context(component: &'static str) -> Result<AvatarContext, AvatarError> {
    try_use_context::<AvatarContext>().ok_or(AvatarError::MissingAvatarProvider { component })
}

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
    // Every Avatar tree gets its own loading-status signal.
    let status = use_signal(|| ImageLoadingStatus::Loading);
    use_context_provider(|| AvatarContext { status });

    // When nested inside an AvatarGroup, add a ring so stacked avatars have a
    // visible border between them.
    let is_grouped = try_use_context::<AvatarGroupCtx>().is_some();
    let group_class = if is_grouped {
        "ring-2 ring-background"
    } else {
        ""
    };

    rsx! {
        span {
            class: format!(
                "relative flex shrink-0 overflow-hidden rounded-full size-10 {} {}",
                group_class,
                props.class,
            ),
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
            class: format!("aspect-square size-full object-cover {}", props.class),
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

    // Once the image has loaded, the fallback gets out of the way.
    if *status.read() == ImageLoadingStatus::Loaded {
        return rsx! {};
    }

    rsx! {
        span {
            class: format!(
                "absolute inset-0 flex items-center justify-center rounded-full bg-muted text-sm font-medium {}",
                props.class,
            ),
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

/// A small status indicator badge positioned at the bottom-right of an Avatar.
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
    // Validates that we are inside an <Avatar>, even though we don't need the
    // status signal — a badge outside an Avatar makes no sense.
    if let Err(e) = use_avatar_context("AvatarBadge") {
        return render_error(e);
    }

    rsx! {
        span {
            class: format!(
                "absolute bottom-0 right-0 block size-3 rounded-full ring-2 ring-background {}",
                props.class,
            ),
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
    // Provide marker context so child Avatars know they are grouped.
    use_context_provider(|| AvatarGroupCtx);

    rsx! {
        div {
            class: format!("flex -space-x-3 {}", props.class),
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
            class: format!(
                "relative flex shrink-0 items-center justify-center rounded-full bg-muted text-xs font-medium size-10 ring-2 ring-background {}",
                props.class,
            ),
            {props.children}
        }
    }
}

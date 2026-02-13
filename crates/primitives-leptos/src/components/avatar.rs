use leptos::prelude::*;
use primitives_core::avatar::{
    AVATAR_BADGE_BASE, AVATAR_BASE, AVATAR_FALLBACK_BASE, AVATAR_GROUP_BASE,
    AVATAR_GROUP_COUNT_BASE, AVATAR_GROUP_RING, AVATAR_IMAGE_BASE, AvatarError, ImageLoadingStatus,
};

// ---------------------------------------------------------------------------
// Context types (Leptos-specific)
// ---------------------------------------------------------------------------

/// Context provided by `Avatar` to all child components.
#[derive(Clone, Copy)]
struct AvatarContext {
    status: ReadSignal<ImageLoadingStatus>,
    set_status: WriteSignal<ImageLoadingStatus>,
}

/// Marker context provided by `AvatarGroup` so that child `Avatar` components
/// can automatically add ring/border styling for the overlapping group look.
#[derive(Clone, Copy)]
struct AvatarGroupCtx;

// ---------------------------------------------------------------------------
// Error rendering (Leptos-specific)
// ---------------------------------------------------------------------------

fn render_error(error: AvatarError) -> impl IntoView {
    #[cfg(debug_assertions)]
    {
        let msg = error.to_string();
        view! {
            <span style="color: #ef4444; background: #fef2f2; border: 1px solid #fca5a5; padding: 4px 8px; border-radius: 4px; font-size: 12px; font-family: monospace;">
                {msg}
            </span>
        }
        .into_any()
    }

    #[cfg(not(debug_assertions))]
    {
        panic!("{error}");
    }
}

// ---------------------------------------------------------------------------
// Avatar (root)
// ---------------------------------------------------------------------------

/// Root wrapper for an avatar. Provides image-loading context to children.
///
/// ```rust
/// view! {
///     <Avatar>
///         <AvatarImage src="https://example.com/photo.png" alt="User" />
///         <AvatarFallback>"UN"</AvatarFallback>
///     </Avatar>
/// }
/// ```
#[component]
pub fn Avatar(
    /// Additional Tailwind classes to merge onto the root element.
    #[prop(optional, into)]
    class: String,
    children: Children,
) -> impl IntoView {
    let (status, set_status) = signal(ImageLoadingStatus::Loading);
    provide_context(AvatarContext { status, set_status });

    let is_grouped = use_context::<AvatarGroupCtx>().is_some();
    let group_class = if is_grouped { AVATAR_GROUP_RING } else { "" };

    let classes = format!("{} {} {}", AVATAR_BASE, group_class, class);

    view! {
        <span class={classes}>
            {children()}
        </span>
    }
}

// ---------------------------------------------------------------------------
// AvatarImage
// ---------------------------------------------------------------------------

/// The `<img>` inside an `Avatar`. Communicates load / error status to the
/// parent context so that `AvatarFallback` can react accordingly.
#[component]
pub fn AvatarImage(
    /// Image source URL.
    #[prop(into)]
    src: String,
    /// Accessible alt text.
    #[prop(optional, into)]
    alt: String,
    /// Additional Tailwind classes for the `<img>` element.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let ctx = use_context::<AvatarContext>();

    match ctx {
        None => render_error(AvatarError::MissingAvatarProvider {
            component: "AvatarImage",
        })
        .into_any(),
        Some(ctx) => {
            let set_status = ctx.set_status;
            let classes = format!("{} {}", AVATAR_IMAGE_BASE, class);

            view! {
                <img
                    class={classes}
                    src={src}
                    alt={alt}
                    on:load=move |_| set_status.set(ImageLoadingStatus::Loaded)
                    on:error=move |_| set_status.set(ImageLoadingStatus::Error)
                />
            }
            .into_any()
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarFallback
// ---------------------------------------------------------------------------

/// Fallback content rendered when the avatar image is still loading or failed.
/// Automatically hides once the image has loaded successfully.
#[component]
pub fn AvatarFallback(
    /// Additional Tailwind classes.
    #[prop(optional, into)]
    class: String,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<AvatarContext>();

    match ctx {
        None => render_error(AvatarError::MissingAvatarProvider {
            component: "AvatarFallback",
        })
        .into_any(),
        Some(ctx) => {
            let status = ctx.status;

            view! {
                <span class={move || {
                    let hidden = if status.get() == ImageLoadingStatus::Loaded { " hidden" } else { "" };
                    format!("{} {}{}", AVATAR_FALLBACK_BASE, class, hidden)
                }}>
                    {children()}
                </span>
            }
            .into_any()
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarBadge
// ---------------------------------------------------------------------------

/// A small status indicator badge positioned at the bottom-end of an Avatar.
///
/// ```rust
/// view! {
///     <Avatar>
///         <AvatarImage src="…" alt="…" />
///         <AvatarFallback>"CN"</AvatarFallback>
///         <AvatarBadge class="bg-green-600" />
///     </Avatar>
/// }
/// ```
#[component]
pub fn AvatarBadge(
    /// Additional Tailwind classes (e.g. `bg-green-600` for an "online" dot).
    #[prop(optional, into)]
    class: String,
    /// Optional children (e.g. an icon inside the badge).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let ctx = use_context::<AvatarContext>();

    match ctx {
        None => render_error(AvatarError::MissingAvatarProvider {
            component: "AvatarBadge",
        })
        .into_any(),
        Some(_) => {
            let classes = format!("{} {}", AVATAR_BADGE_BASE, class);

            view! {
                <span class={classes}>
                    {children.map(|c| c())}
                </span>
            }
            .into_any()
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarGroup
// ---------------------------------------------------------------------------

/// Renders a row of overlapping avatars. Child `Avatar` components
/// automatically receive ring styling when nested inside a group.
///
/// ```rust
/// view! {
///     <AvatarGroup>
///         <Avatar>
///             <AvatarImage src="…" alt="…" />
///             <AvatarFallback>"A"</AvatarFallback>
///         </Avatar>
///         <Avatar>
///             <AvatarImage src="…" alt="…" />
///             <AvatarFallback>"B"</AvatarFallback>
///         </Avatar>
///         <AvatarGroupCount>"+3"</AvatarGroupCount>
///     </AvatarGroup>
/// }
/// ```
#[component]
pub fn AvatarGroup(
    /// Additional Tailwind classes.
    #[prop(optional, into)]
    class: String,
    /// The `Avatar` (and optional `AvatarGroupCount`) children.
    children: Children,
) -> impl IntoView {
    provide_context(AvatarGroupCtx);

    let classes = format!("{} {}", AVATAR_GROUP_BASE, class);

    view! {
        <div class={classes}>
            {children()}
        </div>
    }
}

// ---------------------------------------------------------------------------
// AvatarGroupCount
// ---------------------------------------------------------------------------

/// A pill / circle that shows the number of remaining avatars not displayed in
/// the group.
#[component]
pub fn AvatarGroupCount(
    /// Additional Tailwind classes.
    #[prop(optional, into)]
    class: String,
    /// The count label, e.g. `"+3"`.
    children: Children,
) -> impl IntoView {
    let classes = format!("{} {}", AVATAR_GROUP_COUNT_BASE, class);

    view! {
        <span class={classes}>
            {children()}
        </span>
    }
}

use primitives_core::avatar::{
    AVATAR_BADGE_BASE, AVATAR_BASE, AVATAR_FALLBACK_BASE, AVATAR_GROUP_BASE,
    AVATAR_GROUP_COUNT_BASE, AVATAR_GROUP_RING, AVATAR_IMAGE_BASE, AvatarError, ImageLoadingStatus,
};
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Context types (Yew-specific)
// ---------------------------------------------------------------------------

/// Context provided by `Avatar` to all child components. Wrapping the state in
/// a named struct avoids accidental collisions with unrelated contexts
/// elsewhere in the tree.
#[derive(Clone, PartialEq)]
pub struct AvatarContext {
    pub status: UseStateHandle<ImageLoadingStatus>,
}

/// Marker context provided by `AvatarGroup` so that child `Avatar` components
/// can automatically add ring/border styling for the overlapping group look.
#[derive(Clone, PartialEq)]
pub struct AvatarGroupCtx;

// ---------------------------------------------------------------------------
// Error rendering (Yew-specific)
// ---------------------------------------------------------------------------

fn render_error(error: AvatarError) -> Html {
    #[cfg(debug_assertions)]
    {
        let msg = error.to_string();
        html! {
            <span style="color: #ef4444; background: #fef2f2; border: 1px solid #fca5a5; padding: 4px 8px; border-radius: 4px; font-size: 12px; font-family: monospace;">
                {msg}
            </span>
        }
    }

    #[cfg(not(debug_assertions))]
    {
        panic!("{error}");
    }
}

// ---------------------------------------------------------------------------
// Avatar (root)
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct AvatarProps {
    /// Additional Tailwind classes to merge onto the root element.
    #[prop_or_default]
    pub class: String,
    pub children: Children,
}

/// Root wrapper for an avatar. Provides image-loading context to children.
///
/// ```rust
/// html! {
///     <Avatar>
///         <AvatarImage src="https://example.com/photo.png" alt="User" />
///         <AvatarFallback>{"UN"}</AvatarFallback>
///     </Avatar>
/// }
/// ```
#[component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    let status = use_state(|| ImageLoadingStatus::Loading);
    let ctx = AvatarContext {
        status: status.clone(),
    };

    let is_grouped = use_context::<AvatarGroupCtx>().is_some();
    let group_class = if is_grouped { AVATAR_GROUP_RING } else { "" };

    let classes = format!("{} {} {}", AVATAR_BASE, group_class, props.class);

    html! {
        <ContextProvider<AvatarContext> context={ctx}>
            <span class={classes}>
                { for props.children.iter() }
            </span>
        </ContextProvider<AvatarContext>>
    }
}

// ---------------------------------------------------------------------------
// AvatarImage
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct AvatarImageProps {
    /// Image source URL.
    pub src: String,
    /// Accessible alt text.
    #[prop_or_default]
    pub alt: String,
    /// Additional Tailwind classes for the `<img>` element.
    #[prop_or_default]
    pub class: String,
}

/// The `<img>` inside an `Avatar`. Communicates load / error status to the
/// parent context so that `AvatarFallback` can react accordingly.
#[component(AvatarImage)]
pub fn avatar_image(props: &AvatarImageProps) -> Html {
    let ctx = use_context::<AvatarContext>();

    match ctx {
        None => render_error(AvatarError::MissingAvatarProvider {
            component: "AvatarImage",
        }),
        Some(ctx) => {
            let status = ctx.status.clone();
            let classes = format!("{} {}", AVATAR_IMAGE_BASE, props.class);

            let onload = {
                let status = status.clone();
                Callback::from(move |_: Event| {
                    status.set(ImageLoadingStatus::Loaded);
                })
            };

            let onerror = {
                let status = status.clone();
                Callback::from(move |_: Event| {
                    status.set(ImageLoadingStatus::Error);
                })
            };

            html! {
                <img
                    class={classes}
                    src={props.src.clone()}
                    alt={props.alt.clone()}
                    onload={onload}
                    onerror={onerror}
                />
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarFallback
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct AvatarFallbackProps {
    /// Additional Tailwind classes.
    #[prop_or_default]
    pub class: String,
    /// Typically one or two letters (initials) or an icon.
    pub children: Children,
}

/// Fallback content rendered when the avatar image is still loading or failed.
/// Automatically hides once the image has loaded successfully.
#[component(AvatarFallback)]
pub fn avatar_fallback(props: &AvatarFallbackProps) -> Html {
    let ctx = use_context::<AvatarContext>();

    match ctx {
        None => render_error(AvatarError::MissingAvatarProvider {
            component: "AvatarFallback",
        }),
        Some(ctx) => {
            let status = *ctx.status;

            if status == ImageLoadingStatus::Loaded {
                return html! {};
            }

            let classes = format!("{} {}", AVATAR_FALLBACK_BASE, props.class);

            html! {
                <span class={classes}>
                    { for props.children.iter() }
                </span>
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarBadge
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct AvatarBadgeProps {
    /// Additional Tailwind classes (e.g. `bg-green-600` for an "online" dot).
    #[prop_or_default]
    pub class: String,
    /// Optional children (e.g. an icon inside the badge).
    #[prop_or_default]
    pub children: Children,
}

/// A small status indicator badge positioned at the bottom-end of an Avatar.
///
/// ```rust
/// html! {
///     <Avatar>
///         <AvatarImage src="…" alt="…" />
///         <AvatarFallback>{"CN"}</AvatarFallback>
///         <AvatarBadge class="bg-green-600" />
///     </Avatar>
/// }
/// ```
#[component(AvatarBadge)]
pub fn avatar_badge(props: &AvatarBadgeProps) -> Html {
    let ctx = use_context::<AvatarContext>();

    match ctx {
        None => render_error(AvatarError::MissingAvatarProvider {
            component: "AvatarBadge",
        }),
        Some(_) => {
            let classes = format!("{} {}", AVATAR_BADGE_BASE, props.class);

            html! {
                <span class={classes}>
                    { for props.children.iter() }
                </span>
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarGroup
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct AvatarGroupProps {
    /// Additional Tailwind classes.
    #[prop_or_default]
    pub class: String,
    /// The `Avatar` (and optional `AvatarGroupCount`) children.
    pub children: Children,
}

/// Renders a row of overlapping avatars. Child `Avatar` components
/// automatically receive ring styling when nested inside a group.
///
/// ```rust
/// html! {
///     <AvatarGroup>
///         <Avatar>
///             <AvatarImage src="…" alt="…" />
///             <AvatarFallback>{"A"}</AvatarFallback>
///         </Avatar>
///         <Avatar>
///             <AvatarImage src="…" alt="…" />
///             <AvatarFallback>{"B"}</AvatarFallback>
///         </Avatar>
///         <AvatarGroupCount>{"+3"}</AvatarGroupCount>
///     </AvatarGroup>
/// }
/// ```
#[component(AvatarGroup)]
pub fn avatar_group(props: &AvatarGroupProps) -> Html {
    let classes = format!("{} {}", AVATAR_GROUP_BASE, props.class);

    html! {
        <ContextProvider<AvatarGroupCtx> context={AvatarGroupCtx}>
            <div class={classes}>
                { for props.children.iter() }
            </div>
        </ContextProvider<AvatarGroupCtx>>
    }
}

// ---------------------------------------------------------------------------
// AvatarGroupCount
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct AvatarGroupCountProps {
    /// Additional Tailwind classes.
    #[prop_or_default]
    pub class: String,
    /// The count label, e.g. `"+3"`.
    pub children: Children,
}

/// A pill / circle that shows the number of remaining avatars not displayed in
/// the group.
#[component(AvatarGroupCount)]
pub fn avatar_group_count(props: &AvatarGroupCountProps) -> Html {
    let classes = format!("{} {}", AVATAR_GROUP_COUNT_BASE, props.class);

    html! {
        <span class={classes}>
            { for props.children.iter() }
        </span>
    }
}

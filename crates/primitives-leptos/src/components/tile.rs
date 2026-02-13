use leptos::prelude::*;
use primitives_core::tile::{TILE_BASE, TILE_BODY_BASE, TILE_HEADER_BASE, TileAlignment, TileKind};

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A tile component that renders a structured card-like element with a header
/// row (leading content, label, trailing content) and a body row.
///
/// Uses `primitives_core::tile` for shared enums (`TileKind`,
/// `TileAlignment`) and class constants (`TILE_BASE`, `TILE_HEADER_BASE`,
/// `TILE_BODY_BASE`).
///
/// ## Kinds
///
/// `Selection` (default) · `Action`
///
/// ## Alignments
///
/// `Start` (default) · `Center` · `End`
///
/// ## Example
///
/// ```rust
/// view! {
///     <Tile
///         label="My Tile"
///         kind=TileKind::Selection
///         header_alignment=TileAlignment::Start
///         body_alignment=TileAlignment::Center
///         leading_content=|| view! { <span>"★"</span> }
///         trailing_content=|| view! { <span>"→"</span> }
///     >
///         <p>"Tile body content goes here."</p>
///     </Tile>
/// }
/// ```
#[component]
pub fn Tile(
    /// The label text displayed in the tile header.
    #[prop(into)]
    label: String,
    /// The visual kind of the tile.
    #[prop(optional, into)]
    kind: TileKind,
    /// Alignment of the header row.
    #[prop(optional, into)]
    header_alignment: TileAlignment,
    /// Alignment of the body row.
    #[prop(optional, into)]
    body_alignment: TileAlignment,
    /// Whether the tile is currently selected.
    #[prop(optional)]
    selected: bool,
    /// Additional Tailwind classes merged onto the root element.
    #[prop(optional, into)]
    class: String,
    /// Content rendered before the label in the header row (e.g. an icon).
    #[prop(optional, into)]
    leading_content: Option<ViewFn>,
    /// Content rendered after the label in the header row (e.g. a chevron).
    #[prop(optional, into)]
    trailing_content: Option<ViewFn>,
    /// Tile body content.
    children: Children,
) -> impl IntoView {
    let root_classes = format!("{} {} {}", TILE_BASE, kind.classes(), class);
    let header_classes = format!("{} {}", TILE_HEADER_BASE, header_alignment.classes());
    let body_classes = format!("{} {}", TILE_BODY_BASE, body_alignment.classes());

    let data_kind = kind.as_str();
    let data_header_align = header_alignment.as_str();
    let data_body_align = body_alignment.as_str();

    view! {
        <div
            class={root_classes}
            data-slot="tile"
            data-kind={data_kind}
            data-selected={selected.to_string()}
            aria-selected={selected.to_string()}
        >
            <div class="flex items-center">
                <div
                    class={header_classes}
                    data-slot="tile-header"
                    data-align={data_header_align}
                >
                    {leading_content.map(|lc| lc.run())}
                    <span>{label}</span>
                    {trailing_content.map(|tc| tc.run())}
                </div>
            </div>
            <div
                class={body_classes}
                data-slot="tile-body"
                data-align={data_body_align}
            >
                {children()}
            </div>
        </div>
    }
}

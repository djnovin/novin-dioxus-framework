use primitives_core::tile::{TILE_BASE, TILE_BODY_BASE, TILE_HEADER_BASE, TileAlignment, TileKind};
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct TileProps {
    /// The label text displayed in the tile header.
    pub label: String,

    /// The visual kind of the tile.
    #[prop_or_default]
    pub kind: TileKind,

    /// Alignment of the header row.
    #[prop_or_default]
    pub header_alignment: TileAlignment,

    /// Alignment of the body row.
    #[prop_or_default]
    pub body_alignment: TileAlignment,

    /// Whether the tile is currently selected.
    #[prop_or_default]
    pub selected: bool,

    /// Additional Tailwind classes merged onto the root element.
    #[prop_or_default]
    pub class: String,

    /// Content rendered before the label in the header row (e.g. an icon).
    #[prop_or_default]
    pub leading_content: Option<Html>,

    /// Content rendered after the label in the header row (e.g. a chevron).
    #[prop_or_default]
    pub trailing_content: Option<Html>,

    /// Tile body content.
    pub children: Children,
}

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
/// html! {
///     <Tile
///         label="My Tile"
///         kind={TileKind::Selection}
///         header_alignment={TileAlignment::Start}
///         body_alignment={TileAlignment::Center}
///         leading_content={html! { <span>{"★"}</span> }}
///         trailing_content={html! { <span>{"→"}</span> }}
///     >
///         <p>{"Tile body content goes here."}</p>
///     </Tile>
/// }
/// ```
#[component(Tile)]
pub fn tile(props: &TileProps) -> Html {
    let root_classes = format!("{} {} {}", TILE_BASE, props.kind.classes(), props.class);
    let header_classes = format!("{} {}", TILE_HEADER_BASE, props.header_alignment.classes());
    let body_classes = format!("{} {}", TILE_BODY_BASE, props.body_alignment.classes());

    let data_kind = props.kind.as_str();
    let data_header_align = props.header_alignment.as_str();
    let data_body_align = props.body_alignment.as_str();

    html! {
        <div
            class={root_classes}
            data-slot="tile"
            data-kind={data_kind}
            data-selected={props.selected.to_string()}
            aria-selected={props.selected.to_string()}
        >
            <div class="flex items-center">
                <div
                    class={header_classes}
                    data-slot="tile-header"
                    data-align={data_header_align}
                >
                    if let Some(ref leading) = props.leading_content {
                        { leading.clone() }
                    }
                    <span>{ &props.label }</span>
                    if let Some(ref trailing) = props.trailing_content {
                        { trailing.clone() }
                    }
                </div>
            </div>
            <div
                class={body_classes}
                data-slot="tile-body"
                data-align={data_body_align}
            >
                { for props.children.iter() }
            </div>
        </div>
    }
}

pub mod accordion;
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

pub use accordion::Accordion;
pub use alert::Alert;
pub use avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage};
pub use badge::Badge;
pub use button::Button;
pub use card::Card;
pub use input::Input;
pub use link::Link;
pub use loading::Loading;
pub use section::Section;
pub use separator::Separator;
pub use skeleton::Skeleton;
pub use text_area::TextArea;
pub use tile::Tile;
pub use toggle::Toggle;
pub use tooltip::{Tooltip, TooltipContent, TooltipTrigger};
pub use typography::Typography;

// Re-export core types so consumers don't need to depend on primitives-core
// directly.
pub use primitives_core::alert::AlertVariant;
pub use primitives_core::avatar::{AvatarError, ImageLoadingStatus};
pub use primitives_core::badge::BadgeVariant;
pub use primitives_core::button::{ButtonSize, ButtonVariant};
pub use primitives_core::section::SectionType;
pub use primitives_core::separator::SeparatorOrientation;
pub use primitives_core::tile::{TileAlignment, TileKind};
pub use primitives_core::toggle::{ToggleSize, ToggleVariant};
pub use primitives_core::tooltip::{TooltipError, TooltipSide};
pub use primitives_core::typography::TypographyVariant;

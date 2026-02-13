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

pub use accordion::{Accordion, AccordionProps};
pub use alert::{Alert, AlertProps};
pub use avatar::{
    Avatar, AvatarBadge, AvatarBadgeProps, AvatarFallback, AvatarFallbackProps, AvatarGroup,
    AvatarGroupCount, AvatarGroupCountProps, AvatarGroupProps, AvatarImage, AvatarImageProps,
    AvatarProps,
};
pub use badge::{Badge, BadgeProps};
pub use button::{Button, ButtonProps};
pub use card::{Card, CardProps};
pub use input::{Input, InputProps};
pub use link::{Link, LinkProps};
pub use loading::{Loading, LoadingProps};
pub use section::{Section, SectionProps};
pub use separator::{Separator, SeparatorProps};
pub use skeleton::{Skeleton, SkeletonProps};
pub use text_area::{TextArea, TextAreaProps};
pub use tile::{Tile, TileProps};
pub use toggle::{Toggle, ToggleProps};
pub use tooltip::{
    Tooltip, TooltipContent, TooltipContentProps, TooltipProps, TooltipTrigger, TooltipTriggerProps,
};
pub use typography::{Typography, TypographyProps};

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

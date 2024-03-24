use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TypographyVariant {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Subtitle1,
    Subtitle2,
    Body1,
    Body2,
    Caption,
    Overline,
    Blockquote,
    Muted,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypographyProps {
    pub variant: TypographyVariant,
}

#[component]
pub fn typography(props: TypographyProps, children: Element, class: Option<String>) -> Element {
    let base_class = class.unwrap_or_default();
    let class = match props.variant {
        TypographyVariant::H1 => "text-6xl",
        TypographyVariant::H2 => "text-3xl",
        TypographyVariant::H3 => "text-2xl",
        TypographyVariant::H4 => "text-xl",
        TypographyVariant::H5 => "text-lg ",
        TypographyVariant::H6 => "text-base",
        TypographyVariant::Subtitle1 => "text-lg font-medium uppercase",
        TypographyVariant::Subtitle2 => "text-base font-medium uppercase",
        TypographyVariant::Body1 => "text-lg",
        TypographyVariant::Body2 => "text-base",
        TypographyVariant::Caption => "text-sm",
        TypographyVariant::Overline => "text-xs uppercase tracking-widest font-medium",
        TypographyVariant::Blockquote => "mt-6 border-l-2 pl-6 italic",
        TypographyVariant::Muted => "text-sm text-muted-foreground",
    };

    rsx! {
        if props.variant == TypographyVariant::H1 {
            h1 {
                class: format!("{} {}", base_class, class),
                {children}
            }
        } else if props.variant == TypographyVariant::H2 {
            h2 {
                class: format!("{} {}", base_class, class),
                {children}
            }
        } else if props.variant == TypographyVariant::H3 {
            h3 {
                class: format!("{} {}", base_class, class),
                {children}
            }
        } else if props.variant == TypographyVariant::H4 {
            h4 {
                class: format!("{} {}", base_class, class),
                {children}
            }
        } else if props.variant == TypographyVariant::H5 {
            h5 {
                class: format!("{} {}", base_class, class),
                {children}
            }
        } else if props.variant == TypographyVariant::H6 {
            h6 {
                class: format!("{} {}", base_class, class),
                {children}
            }
        } else if props.variant == TypographyVariant::Subtitle1 {
            h6 {
                class: format!("{} {}", base_class, class),
                {children}
            }
        } else if props.variant == TypographyVariant::Subtitle2 {
            h6 {
                class: format!("{} {}", base_class, class),
                {children}
            }
        } else if props.variant == TypographyVariant::Body1 {
            p {
                class: format!("{} {}", base_class, class),
                {children}
            }
        } else if props.variant == TypographyVariant::Body2 {
            p {
                class: format!("{} {}", base_class, class),
                {children}
            }
        } else if props.variant == TypographyVariant::Caption {
            p {
                class: format!("{} {}", base_class, class),
                {children}
            }
        } else if props.variant == TypographyVariant::Overline {
            p {
                class: format!("{} {}", base_class, class),
                {children}
            }
        } else if props.variant == TypographyVariant::Blockquote {
            blockquote {
                class: format!("{} {}", base_class, class),
                {children}
            }
        } else if props.variant == TypographyVariant::Muted {
            p {
                class: format!("{} {}", base_class, class),
                {children}
            }
        }
    }
}

use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum BadgeVariants {
    Primary,
    Secondary,
    Outline,
    Destructive,
}

#[component]
pub fn badge(children: Element, variant: BadgeVariants) -> Element {
    let base_class = "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2";

    let badge_variant_class = match variant {
        BadgeVariants::Primary => "border-transparent bg-primary text-primary-foreground shadow hover:bg-primary/80",
        BadgeVariants::Secondary => "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80",
        BadgeVariants::Outline => "text-foreground",
        BadgeVariants::Destructive => "border-transparent bg-destructive text-destructive-foreground shadow hover:bg-destructive/80",
    };

    let badge_class = format!("{} {}", base_class, badge_variant_class);

    rsx! {
        span {
            class: badge_class,
            {children}
        }
    }
}

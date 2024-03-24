use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum AvatarSize {
    Small,
    Medium,
    Large,
}

#[component]
pub fn avatar(size: AvatarSize, children: Element) -> Element {
    let base_class = "bg-gray-200 rounded-full flex items-center justify-center";
    let avatar_class = match size {
        AvatarSize::Small => format!("{} w-8 h-8", base_class),
        AvatarSize::Medium => format!("{} w-12 h-12", base_class),
        AvatarSize::Large => format!("{} w-16 h-16", base_class),
    };

    rsx! {
        div {
            class: avatar_class,
            {children}
        }
    }
}

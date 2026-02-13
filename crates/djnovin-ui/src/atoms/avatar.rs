use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum AvatarSize {
    Small,
    Medium,
    Large,
}

#[derive(Props, PartialEq, Clone)]
pub struct AvatarProps {
    size: AvatarSize,
    children: Element,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let base_class = "bg-gray-200 rounded-full flex items-center justify-center";
    let avatar_class = match props.size {
        AvatarSize::Small => format!("{} w-8 h-8", base_class),
        AvatarSize::Medium => format!("{} w-12 h-12", base_class),
        AvatarSize::Large => format!("{} w-16 h-16", base_class),
    };

    rsx! {
        div { class: avatar_class, {props.children} }
    }
}

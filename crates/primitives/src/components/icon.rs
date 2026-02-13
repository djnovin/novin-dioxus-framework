enum Size {
    Small,
    Medium,
    Large,
}

pub enum IconName {
    Home = rsx! {<path d="M12 14v-2a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v2" />},
    User = rsx! {<path d="M12 14v-2a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v2" />},
    Settings = rsx! {<path d="M12 14v-2a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v2" />},
}

#[derive(Prop, PartialEq, Clone)]
pub struct IconProps {
    name: IconName,
    size: Size,
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    let name = match props.name {
        IconName::Home => "home",
        IconName::User => "user",
        IconName::Settings => "settings",
    };

    let size = match props.size {
        Size::Small => "16",
        Size::Medium => "24",
        Size::Large => "32",
    };

    rsx! {
        svg {
            class={format!("w-{size} h-{size}", size=size)}
            {name}
        }
    }
}

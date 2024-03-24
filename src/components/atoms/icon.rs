enum Size {
    sm,
    md,
    lg,
}

enum IconName {
    home = rsx! {<path d="M12 14v-2a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v2" />},
    user = rsx! {<path d="M12 14v-2a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v2" />},
    settings = rsx! {<path d="M12 14v-2a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v2" />},
}

struct IconProps {
    name: IconName,
    size: Size,
}

#[component]
fn Icon(props: IconProps) -> Element {
    let name = match props.name {
        IconName::home => "home",
        IconName::user => "user",
        IconName::settings => "settings",
    };

    let size = match props.size {
        Size::sm => "16",
        Size::md => "24",
        Size::lg => "32",
    };

    rsx! {
        svg {
            class={format!("w-{size} h-{size}", size=size)}
            {name}
        }
    }
}

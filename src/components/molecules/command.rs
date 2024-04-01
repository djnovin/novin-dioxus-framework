use dioxus::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct CommandProps {
    pub children: Element,
    pub open: bool,
    pub on_open_change: Callback<String>,
}

#[component]
pub fn command(props: CommandProps) -> Element {
    let mut is_open = use_signal(props.open);

    use_key_press(|event| {
        if event.key() == "Enter" {
            is_open.set(!is_open.get());
            props.on_open_change.emit("open".to_string());
        }
    });

    rsx! {
        if open == true {
            div {
                class: "flex space-x-4"
                children: props.children
            }
       } else {
            div {}
       }
    }
}

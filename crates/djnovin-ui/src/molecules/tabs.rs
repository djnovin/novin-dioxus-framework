use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct TabItems {
    label: String,
    children: Element,
}

#[derive(PartialEq, Clone)]
pub enum TabOrientation {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Clone)]
pub struct TabProps {
    tabs: Vec<TabItems>,
    orientation: TabOrientation,
    active_key: usize,
    disabled: bool,
}

#[component]
pub fn tabs(props: TabProps) -> Element {
    let mut active_key = use_signal(|| props.active_key);

    let base_class = "relative";

    let orientation_class = match props.orientation {
        TabOrientation::Horizontal => "flex flex-row",
        TabOrientation::Vertical => "flex flex-col",
    };

    rsx! {
        div {
            class: format!("{} {}", base_class, orientation_class),
            {
                rsx! {
                    div {
                        class: "flex items-center space-x-2",
                        {
                            for (index, tab) in props.tabs.iter().enumerate() {
                                rsx! {
                                    div {
                                        class: "flex items-center space-x-2 p-2",
                                        {
                                            rsx! {
                                                button {
                                                    class: "text-sm font-medium text-gray-500",
                                                    onclick: move |_| {
                                                        props.active_key = index;
                                                    },
                                                    {tab.label}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

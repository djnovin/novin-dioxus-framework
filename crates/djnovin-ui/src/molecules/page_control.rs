use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct PageControlProps {
    total: usize,
    current: usize,
    on_change: Callback<usize>,
    aria_label: Option<String>,
    disabled: Option<bool>,
}

#[component]
pub fn page_control(props: PageControlProps) -> Element {
    let total = use_signal(|| props.total);
    let current = use_signal(|| props.current);

    let base_class = "flex items-center space-x-2";

    rsx! {
        div {
            aria_label: props.aria_label,
            class: base_class,
            {
                rsx! {
                    button {
                        class: "text-sm font-medium text-gray-500",
                        onclick: move |_| {
                            if current > 1 {
                                props.on_change.emit(current - 1);
                            }
                        },
                        {"<"}
                    }
                    span {
                        class: "text-sm font-medium text-gray-500",
                        {current.to_string()}
                    }
                    button {
                        class: "text-sm font-medium text-gray-500",
                        onclick: move |_| {
                            if current < total {
                                props.on_change.emit(current + 1);
                            }
                        },
                        {">"}
                    }
                }
            }
        }
    }
}

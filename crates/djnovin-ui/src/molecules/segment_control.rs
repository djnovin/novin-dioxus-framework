use dioxus::prelude::*;

pub struct SegmentControlProps {
    pub segments: Vec<String>,
    pub active_key: String,
    pub on_change: Callback<String>,
}

#[component]
pub fn segment_control(props: SegmentControlProps)  -> Element {
    rsx! {
        div {
            class: "flex space-x-4",
            {
                for (index, segment) in props.segments.iter().enumerate() {
                    let is_active = props.active_key == *segment;
                    let class = if is_active {
                        "bg-gray-900 text-white"
                    } else {
                        "bg-gray-200 text-gray-900"
                    };
                    rsx! {
                        button {
                            class: format!("px-4 py-2 rounded-lg {}", class),
                            {
                                event: Click => props.on_change.emit(segment.clone()),
                                {
                                    rsx! {
                                        text! { segment
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

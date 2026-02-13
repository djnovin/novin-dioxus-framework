use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
struct SkeletonProps {
    class: String,
}

#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let base_class = "animate-pulse bg-gray-200";
    let skeleton_class = format!("{}", base_class);

    rsx! {
        div { class: format!("{} {}", skeleton_class, props.class) }
    }
}

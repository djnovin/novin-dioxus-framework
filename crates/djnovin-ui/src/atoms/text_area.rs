use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TextAreaProps {
    pub value: String,
    pub placeholder: String,
}

#[component]
pub fn TextArea(props: TextAreaProps) -> Element {
    let value = props.value;
    let placeholder = props.placeholder;

    let base_class = "flex min-h-[60px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";

    rsx! {
        textarea {
            class: base_class,
            value,
            placeholder,
            on_change,
        }
    }
}

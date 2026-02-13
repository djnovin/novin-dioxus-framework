use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct InputProps {
    pub value: String,
    pub placeholder: String,
}

#[component]
pub fn input(props: InputProps) -> Element {
    let value = props.value;
    let placeholder = props.placeholder;

    let base_class = "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
    rsx! {
        input {
            class: base_class,
            value,
            placeholder,
        }
    }
}

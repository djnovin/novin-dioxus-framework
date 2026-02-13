use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TextAreaProps {
    pub value: String,
    pub placeholder: String,
    pub onblur: Option<EventHandler<FocusEvent>>,
    pub onchange: Option<EventHandler<String>>,
    pub onfocus: Option<EventHandler<FocusEvent>>,
    pub oninput: Option<EventHandler<String>>,
    pub onkeydown: Option<EventHandler<KeyboardEvent>>,
    pub onkeyup: Option<EventHandler<KeyboardEvent>>,
}

#[component]
pub fn TextArea(props: TextAreaProps) -> Element {
    let mut text = use_signal(|| props.value.clone());
    let placeholder = props.placeholder;

    let base_class = "flex min-h-[60px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";

    rsx! {
        textarea {
            class: base_class,
            value: "{text}",
            placeholder: "{placeholder}",
            onblur: move |e| {
                if let Some(onblur) = &props.onblur {
                    onblur.call(e);
                }
            },
            onchange: move |e| {
                let value = e.value().clone();
                text.set(value.clone());
                if let Some(onchange) = &props.onchange {
                    onchange.call(value);
                }
            },
            onfocus: move |e| {
                if let Some(onfocus) = &props.onfocus {
                    onfocus.call(e);
                }
            },
            oninput: move |e| {
                let value = e.value().clone();
                text.set(value.clone());
                if let Some(oninput) = &props.oninput {
                    oninput.call(value);
                }
            },
            onkeydown: move |e| {
                if let Some(onkeydown) = &props.onkeydown {
                    onkeydown.call(e);
                }
            },
            onkeyup: move |e| {
                if let Some(onkeyup) = &props.onkeyup {
                    onkeyup.call(e);
                }
            },
        }
    }
}

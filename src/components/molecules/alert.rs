use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum AlertVariant {
    Error,
    Info,
    Success,
    Warning,
}

#[component]
pub fn alert(variant: AlertVariant, children: Element) -> Element {
    let base_class = "p-4 bg-white rounded-sm border border-gray-200 shadow-sm text-black w-full";
    let alert_class = match variant {
        AlertVariant::Success => format!(
            "{} bg-green-100 border-green-200 text-green-800",
            base_class
        ),
        AlertVariant::Error => format!("{} bg-red-100 border-red-200 text-red-800", base_class),
        AlertVariant::Warning => format!(
            "{} bg-yellow-100 border-yellow-200 text-yellow-800",
            base_class
        ),
        AlertVariant::Info => format!(
            "{} bg-gray-50 border-gray-200 border text-black",
            base_class
        ),
    };

    rsx! {
        div {
            class: alert_class,
            {children}
        }
    }
}

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    #[props(optional)]
    pub color: String,
}
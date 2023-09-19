use dioxus::prelude::*;

#[inline_props]
pub fn LabelAndValue(cx: Scope, label: String, value: String) -> Element {
    render! {
        div {
            style: "margin-bottom: 24px",
            div {
                style: "font-weight: 600; font-color: gray;",
                "{label} :"
            }
            div { "{value}" }
        }
    }
}

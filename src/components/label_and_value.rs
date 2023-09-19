use dioxus::prelude::*;

#[inline_props]
pub fn LabelAndValue(cx: Scope, label: String, value: String) -> Element {
    render! {
        div { "{label}" }
        div { "{value}" }
    }
}

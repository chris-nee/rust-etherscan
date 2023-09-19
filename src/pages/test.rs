use dioxus::prelude::*;

#[inline_props]
pub fn Test(cx: Scope) -> Element {
    render! {
        h1 { "Welcome to Test Page!" }
    }
}

use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

use crate::routes::Route;

use crate::components::custom_button::CustomButton;
use crate::components::custom_input::CustomInput;

#[inline_props]
pub fn ExplorerBox(cx: Scope) -> Element {
    let address = use_state(cx, || "".to_string());
    let nav = use_navigator(cx);
    render! {
        div {
            class: "border border-gray-500 shadow-md w-96 h-60 rounded px-8 py-4",
            div {
                class: "font-bold text-center mt-4 mb-8",
                "The Ethereum Blockchain Explorer"
            }
            CustomInput { value: address, on_change:move |event: FormEvent| {address.set(event.value.clone());} }
            CustomButton { on_click: move |evt| {
                nav.push(Route::Address{ address: String::from(address.to_string())});
            } }
        }
    }
}

#[inline_props]
pub fn Home(cx: Scope) -> Element {
    render! {
        div {
            style: "margin-top: 44px",
            div {
                class: "flex mt-8",
                div {
                    class: "m-auto",
                    ExplorerBox {}
                }
            }
        }
    }
}

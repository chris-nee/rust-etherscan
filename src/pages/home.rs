use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

// Routes
use crate::routes::Route;

// API
use crate::api::etherscan::get_ether_last_price;

// Copmonents
use crate::components::custom_button::CustomButton;
use crate::components::custom_input::CustomInput;

#[inline_props]
pub fn EtherCountDisplay(cx: Scope) -> Element {
    render! {
        div {
            div {
                "Ether Supply"
            }
            div {
                "Ether 2 Supply"
            }
        }
    }
}

#[inline_props]
pub fn EtherPriceDisplay(cx: Scope) -> Element {
    let etherPriceDetails = use_future(cx, (), |_| get_ether_last_price());
    match etherPriceDetails.value() {
        Some(Ok(transaction)) => {
            render! {
                span {
                    style: "font-weight: 400;",
                    "{transaction.result.ethusd}"
                }
            }
        }
        Some(Err(err)) => {
            render! {
                "Error Loading Last Ether Price , {err}"
            }
        }
        _ => {
            render! {"Loading..."}
        }
    }
}

#[inline_props]
pub fn Dashboard(cx: Scope) -> Element {
    render! {
        div {
            class: "border border-gray-500 shadow-md rounded",
            style: "display: flex; text-align: center; height: 340px;",
            div {
                style: "font-weight: 600; flex-grow: 1; padding: 24px;",
                class: "rounded",
                "Ether Price : "
                EtherPriceDisplay {}
            }
            div {
                style: "flex-grow: 1; padding: 24px; border-right: 1px solid gray; border-left: 1px solid gray;",
                class: "rounded",
                "Total Ether Count"
                EtherCountDisplay {}
            }
            div {
                style: "flex-grow: 1; padding: 24px;",
                "Ether Power"
            }
        }
    }
}

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
            style: "margin-top: 44px; display: flex; flex-direction: column;",
            div {
                class: "flex mt-8",
                div {
                    class: "m-auto",
                    ExplorerBox {}
                }
            }
            div {
                class: "flex",
                style: "margin-top: 64px; flex-grow: 1;",
                div {
                    style: "width: 80%;",
                    class: "m-auto",
                    Dashboard {}
                }
            }
        }
    }
}

use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

// Routes
use crate::routes::Route;

// API
use crate::api::etherscan::get_ether2_total_supply;
use crate::api::etherscan::get_ether_last_price;
use crate::api::etherscan::get_ether_node_count;
use crate::api::etherscan::get_ether_total_supply;

// Copmonents
use crate::components::custom_button::CustomButton;
use crate::components::custom_input::CustomInput;

#[inline_props]
pub fn EtherNodeCountDisplay(cx: Scope) -> Element {
    let etherNodeCountDetails = use_future(cx, (), |_| get_ether_node_count());
    match etherNodeCountDetails.value() {
        Some(Ok(etherNodeCountRes)) => {
            render! {
                "{etherNodeCountRes.result.TotalNodeCount}"
            }
        }
        Some(Err(err)) => {
            render! {
                "Error Loading Ether Node Count {err}"
            }
        }
        _ => {
            render! {
                "Loading..."
            }
        }
    }
}

#[inline_props]
pub fn TotalEther2SupplyDisplay(cx: Scope) -> Element {
    let ether2SupplyDetails = use_future(cx, (), |_| get_ether2_total_supply());
    match ether2SupplyDetails.value() {
        Some(Ok(ether2SupplyRes)) => {
            render! {
                "Ether 2 Supply: {ether2SupplyRes.result.Eth2Staking}"
            }
        }
        Some(Err(err)) => {
            render! {
                "Error Loading Ether 2 Supply {err}"
            }
        }
        _ => {
            render! {
                "Loading..."
            }
        }
    }
}

#[inline_props]
pub fn TotalEtherSupplyDisplay(cx: Scope) -> Element {
    let etherSupplyDetails = use_future(cx, (), |_| get_ether_total_supply());
    match etherSupplyDetails.value() {
        Some(Ok(etherSupplyRes)) => {
            render! {
                "Ether Supply: {etherSupplyRes.result}"
            }
        }
        Some(Err(err)) => {
            render! {
                "Error Loading Ether Supply {err}"
            }
        }
        _ => {
            render! {
                "Loading..."
            }
        }
    }
}

#[inline_props]
pub fn EtherCountDisplay(cx: Scope) -> Element {
    render! {
        div {
            style: "font-weight: 400;",
            div {
                TotalEtherSupplyDisplay { }
            }
            div {
                TotalEther2SupplyDisplay { }
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
                    "$ {transaction.result.ethusd}"
                }
            }
        }
        Some(Err(err)) => {
            render! {
                span {
                    style: "font-weight: 400;",
                    "Error Loading Last Ether Price , {err}"
                }
            }
        }
        _ => {
            render! {
                span {
                    style: "font-weight: 400;",
                    "Loading..."
                }
            }
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
                style: "font-weight: 600; flex: 1; padding: 24px;",
                class: "rounded",
                "Ether Price : "
                EtherPriceDisplay {}
            }
            div {
                style: "font-weight: 600; flex: 1; padding: 24px; border-right: 1px solid gray; border-left: 1px solid gray;",
                class: "rounded",
                "Total Ether Count : "
                EtherCountDisplay {}
            }
            div {
                style: "flex: 1; padding: 24px;",
                "Ether Node Count : "
                EtherNodeCountDisplay {}
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

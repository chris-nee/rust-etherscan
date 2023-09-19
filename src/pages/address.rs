use dioxus::prelude::*;

// API
use crate::api::etherscan::{get_transaction_by_hash, TransactionItem};

// Components
use crate::components::label_and_value::LabelAndValue;

#[inline_props]
pub fn AddressDetails(cx: Scope, details: TransactionItem) -> Element {
    render! {
        div {
            class: "font-bold",
            style: "margin-top: 12px;",
            "Transaction Details"
        }
        div {
            style: "border-top: solid 1px gray; margin-top: 24px; padding-top: 12px;",
            LabelAndValue { label: String::from("Transaction Hash"), value: details.result.hash.clone() }
            LabelAndValue { label: String::from("Block"), value: details.result.blockHash.clone() }
        }

        div {
            style: "border-top: solid 1px gray; margin-top: 24px; padding-top: 12px;",
            LabelAndValue { label: String::from("From"), value: details.result.from.clone() }
            LabelAndValue { label: String::from("To"), value: details.result.to.clone() }
        }

        div {
            style: "border-top: solid 1px gray; margin-top: 24px; padding-top: 12px;",
            LabelAndValue { label: String::from("Value"), value: details.result.value.clone() }
            LabelAndValue { label: String::from("Transaction Fee"), value: details.result.gas.clone() }
            LabelAndValue { label: String::from("Gas Price"), value: details.result.gasPrice.clone() }
        }
    }
}

#[inline_props]
pub fn AddressSection(cx: Scope, address: String) -> Element {
    let transaction = use_future(cx, (), |_| get_transaction_by_hash(address.clone()));
    match transaction.value() {
        Some(Ok(transaction)) => {
            render! {
                AddressDetails { details: transaction.clone() }
            }
        }
        Some(Err(err)) => {
            render! {
                "Error loading Tansaction Details {err}"
            }
        }
        _ => {
            render! {"Loading Transaction Details"}
        }
    }
}

#[inline_props]
pub fn Address(cx: Scope, address: Option<String>) -> Element {
    match address {
        Some(add) => {
            render! {
                div {
                    style: "margin-top: 44px",
                    div {
                        class: "flex",
                        div {
                            class: "m-auto",
                            div {
                                class: "border border-gray-500 shadow-md w-4/5 h-60 rounded px-8 py-4",
                                "style": "min-height: 680px; width: 680px;",
                                AddressSection { address: add.clone() }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            render! {
               "Address not found !"
            }
        }
    }
}

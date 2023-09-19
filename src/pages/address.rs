use dioxus::prelude::*;
use std::i64;

// API
use crate::api::etherscan::{get_transaction_by_hash, TransactionItem};

// Components
use crate::components::label_and_value::LabelAndValue;

pub fn hex_str_to_dec_str(hex: String) -> String {
    let without_prefix = hex.trim_start_matches("0x");
    let z = i64::from_str_radix(without_prefix, 16);
    z.unwrap().to_string()
}

#[inline_props]
pub fn AddressDetails(cx: Scope, details: TransactionItem) -> Element {
    let gasPriceStr = details.result.gasPrice.clone();
    let gasPriceStrTrim = gasPriceStr.trim_start_matches("0x");
    let gasPrice = i64::from_str_radix(gasPriceStrTrim, 16);

    let gasStr = details.result.gas.clone();
    let gasStrTrim = gasStr.trim_start_matches("0x");
    let gas = i64::from_str_radix(gasStrTrim, 16);

    let transactionFee = gas.unwrap() * gasPrice.unwrap();
    let transactionFeeStr = transactionFee.to_string();

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
            LabelAndValue { label: String::from("Value"), value: hex_str_to_dec_str(details.result.value.clone()) + " wei" }
            LabelAndValue { label: String::from("Transaction Fee"), value: transactionFeeStr.clone() + " wei" }
            LabelAndValue { label: String::from("Gas Price"), value: hex_str_to_dec_str(details.result.gasPrice.clone()) + " wei" }
            LabelAndValue { label: String::from("Gas Usage"), value: hex_str_to_dec_str(details.result.gas.clone()) }
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
                                class: "border border-gray-500 shadow-md rounded px-8 py-4",
                                "style": "min-height: 700px; width: 680px;",
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

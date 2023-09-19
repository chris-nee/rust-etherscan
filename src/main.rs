#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod api;
mod components;
mod constants;
mod pages;
mod routes;

use routes::Route;

fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

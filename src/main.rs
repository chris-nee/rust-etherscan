#![allow(non_snake_case)]

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};

mod api;
mod components;
mod constants;
mod pages;
mod routes;

use api::stories::get_stories;
use api::stories::StoryItem;

use routes::Route;

#[inline_props]
fn StoryListing(cx: Scope, story: StoryItem) -> Element {
    render!(div {

        story.title.clone()

    })
}

fn Stories(cx: Scope) -> Element {
    let story = use_future(cx, (), |_| get_stories(10));

    match story.value() {
        Some(Ok(list)) => render! {
            div {
                "Stories"
                for story in list {
                    StoryListing { story: story.clone() }
                }
            }
        },
        Some(Err(err)) => render! {"An error occurred while fetching stories {err}"},
        None => render! {"Loading items"},
    }
}

fn Button(cx: Scope) -> Element {
    render!( button {
        class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
        "Click Me"
    })
}

fn List(cx: Scope) -> Element {
    let title = String::from("Name of the List");
    render!( div {
        class:"text-lg font-semibold text-slate-500",
        "{title}"
        }
        Stories{}
    )
}

fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

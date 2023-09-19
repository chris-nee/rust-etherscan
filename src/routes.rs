#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::address::Address;
use crate::pages::home::Home;
use crate::pages::page_not_found::PageNotFound;

use crate::components::layout::HeaderFooter;

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
pub enum Route {
    // If the name of the component and variant are the same you can omit the component and props name
    // If they are different you can specify them like this:
    // #[route("/", ComponentName, PropsName)]

    // The home page is at the / route
    #[layout(HeaderFooter)]
    #[route("/")]
    Home {},
    #[route("/address/:address")]
    Address { address: String },
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

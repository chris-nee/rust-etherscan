use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::routes::Route;

#[inline_props]
pub fn Header(cx: Scope) -> Element {
    render! {
        header {
            nav {
                class: "relative flex flex-wrap items-center justify-between px-2 py-3 bg-cyan-500 mb-3",
                div {
                    class:"w-full relative flex justify-between lg:w-auto  px-4 lg:static lg:block lg:justify-start",
                    a {
                        class: "text-sm font-bold leading-relaxed inline-block mr-4 py-2 whitespace-nowrap uppercase text-white",
                        Link { to: Route::Home {}, "Home" }
                    }
                    a {
                        class: "text-sm font-bold leading-relaxed inline-block mr-4 py-2 whitespace-nowrap uppercase text-white",
                        Link { to: Route::Address {address: String::from("")}, "Address" }
                    }
                }
            }
        }
    }
}

#[inline_props]
pub fn Footer(cx: Scope) -> Element {
    render! {
        footer { "Footer"}
    }
}

#[inline_props]
pub fn Body(cx: Scope) -> Element {
    render! {
        div {
            class: "grow bg-white",
            Outlet::<Route> {}
        }
    }
}

#[inline_props]
pub fn HeaderFooter(cx: Scope) -> Element {
    render! {
        Header{}
        Body {}
        Footer{}
    }
}

use dioxus::prelude::*;

#[inline_props]
pub fn CustomButton<'a>(cx: Scope, on_click: EventHandler<'a, MouseEvent>) -> Element<'a> {
    render! {
        div {
            button {
                class: "w-full bg-cyan-500 text-white active:bg-cyan-600 font-bold uppercase text-xs px-4 py-2 rounded shadow hover:shadow-md outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150",
                onclick: move |evt| {
                    on_click.call(evt)
                },
                "Search",
            }
        }
    }
}

use dioxus::prelude::*;

#[inline_props]
pub fn CustomInput<'a>(
    cx: Scope,
    value: &'a str,
    on_change: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    render! {
        div {
            class:"mb-3 pt-0",
            input {
                class:"h-12 px-2 py-1 placeholder-slate-300 text-slate-600 relative bg-white bg-white rounded text-sm border border-gray-500 shadow outline-none focus:outline-none focus:ring w-full",
                placeholder: "Search by Address",
                value: "{value}",
                oninput: move |event| on_change.call(event)
            }
        }
    }
}

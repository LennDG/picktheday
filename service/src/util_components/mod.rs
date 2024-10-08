use crate::htmx_helpers::{HtmxId, HtmxInput};
use std::fmt::Display;

use leptos::prelude::*;

#[component]
pub fn HiddenInput(id: HtmxId, name: String, value: impl Display) -> impl IntoView {
    view! { <input type="hidden" id=id.to_string() name=name value=value.to_string()/> }
}

#[component]
pub fn HtmxHiddenInput(input: HtmxInput, value: impl Display) -> impl IntoView {
    view! { <input type="hidden" id=input.id.to_string() name=input.name value=value.to_string()/> }
}

// TODO: This effectively duplicates divs, making it not perfect. Visually it works but it's a bit nasty
// e.g.
// <div id=calendar class= ...>
// Becomes
// <div id=calendar class= ...>
//      <div id=calendar class= ...>
#[component]
pub fn HtmxSwapOob(id: HtmxId, children: Children) -> impl IntoView {
    view! {
        <div id=id.to_string() hx-swap-oob="innerHTML">
            {children()}
        </div>
    }
}

// TODO
#[component]
pub fn TextInput(post_uri: String) -> impl IntoView {
    view! {
        <form
            hx-post="/plan"
            class="container relative z-0 mx-auto flex max-w-80 justify-center space-x-4"
        >
            <div>
                <input
                    type="text"
                    name="plan_name"
                    class="border-1 peer block w-full appearance-none rounded-lg border border-gray-600 bg-transparent px-2 py-2.5 text-sm text-white outline-none focus:border-gray-300 "
                    placeholder="e.g. Tennis"
                />
            </div>
            <button
                type="submit"
                class="mb-2 me-2 flex rounded-lg border-gray-700 bg-gray-600 px-5 py-2.5 text-sm font-medium text-white hover:bg-gray-700"
            >
                "Create"
            </button>
        </form>
    }
}

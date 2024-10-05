use crate::htmx_helpers::{HtmxId, HtmxInput};
use std::fmt::Display;

use leptos::prelude::*;

#[component]
pub fn HiddenInput(id: HtmxId, name: String, value: impl Display) -> impl IntoView {
    view! { <input type="hidden" id=id.to_string() name=name value=value.to_string() /> }
}

#[component]
pub fn HtmxHiddenInput(input: HtmxInput, value: impl Display) -> impl IntoView {
    view! { <input type="hidden" id=input.id.to_string() name=input.name value=value.to_string() /> }
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

use crate::htmx_helpers::{HtmxId, HtmxInput};
use std::fmt::Display;

use derive_more::derive::Display;
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

#[derive(Debug, Clone, Copy, Display)]
pub enum Icon {
    #[display("/icons/share.svg")]
    Share,
    #[display("/icons/results.svg")]
    Results,
    #[display("/icons/edit.svg")]
    Edit,
    #[display("/icons/delete.svg")]
    Delete,
    #[display("/icons/arrow_back.svg")]
    Back,
    #[display("/icons/arrow_forward.svg")]
    Forward,
}

#[component]
pub fn Icon(icon: Icon) -> impl IntoView {
    view! {
        <div class="flex items-center justify-center bg-gray-600 p-1 rounded-lg hover:bg-gray-700">
            <img src=icon.to_string()/>
        </div>
    }
}

#[component]
pub fn CopyToClipboard(value: impl Display, children: Children) -> impl IntoView {
    let button_id = HtmxId::new("copy_button").to_string();
    let script = format!("initClipboard('{}', '{}');", button_id, value);

    view! {
        <button id=button_id type="button">
            {children()}
        </button>
        <script>{script}</script>
    }
}

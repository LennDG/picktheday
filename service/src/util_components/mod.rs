use crate::htmx_helpers::{HtmxId, HtmxInput};
use std::fmt::Display;

use leptos::prelude::*;

// TODO! LINK HTMXID AND INPUT NAMES!

#[component]
pub fn HiddenInput(id: HtmxId, name: String, value: impl Display) -> impl IntoView {
    view! { <input type="hidden" id=id.to_string() name=name value=value.to_string() /> }
}

#[component]
pub fn HtmxHiddenInput(input: HtmxInput, value: impl Display) -> impl IntoView {
    view! { <input type="hidden" id=input.id.to_string() name=input.name value=value.to_string() /> }
}

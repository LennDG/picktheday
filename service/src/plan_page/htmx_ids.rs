use crate::htmx_helpers::{HtmxId, HtmxInput};
use once_cell::sync::Lazy;

/// Defines htmx input ids that exist on the plan page which are required by non-related elements
/// Elements that are children should just be passed ids of parents if necessary

// region:	  --- Global htmx inputs
pub static USER_PUBLIC_ID: Lazy<HtmxInput> =
    Lazy::new(|| HtmxInput::new(HtmxId::new("user_public_id"), "user_public_id"));
// endregion: --- Global htmx inputs

// region:	  --- Global htmx IDs
pub static CALENDAR_ID: Lazy<HtmxId> = Lazy::new(|| HtmxId::new("calendar"));
// endregion: --- Global htmx IDs

use axum::{
    debug_handler,
    extract::{Path, State},
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use entity::{db::ModelManager, plans, types::PublicId, users};
use leptos::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};
use time::{Date, Month, OffsetDateTime, Weekday};
use tracing::info;

use crate::{
    error::{Error, Result},
    htmx_helpers::{HtmxId, HtmxInclude, HtmxInput, HtmxTarget},
    plan_page::htmx_ids,
    util_components::HtmxHiddenInput,
};

pub fn routes(mm: ModelManager) -> Router<entity::db::ModelManager> {
    Router::new().nest(
        "/calendar",
        Router::new()
            .route("/", get(get_calendar_handler))
            .with_state(mm),
    )
}

// region:	  --- Calendar handler

#[derive(Debug, Deserialize)]
struct CalendarGet {
    #[serde(deserialize_with = "deserialize_month")]
    month: Month,
    year: i32,
    user_public_id: Option<PublicId>,
}

#[debug_handler]
async fn get_calendar_handler(
    State(mm): State<ModelManager>,
    Path(plan_public_id): Path<PublicId>,
    Form(calendar_get): Form<CalendarGet>,
) -> Result<impl IntoResponse> {
    info!("{:<12} - calendar - {plan_public_id}", "HANDLER");

    // -- Calendar Month (Can I serde that?)
    let calendar_month = CalendarMonth {
        month: calendar_get.month,
        year: calendar_get.year,
    };

    // -- Get the plan
    let plan = plans::helpers::plan_by_public_id(plan_public_id, mm.clone()).await?;

    // -- Get the user if there is one
    let user = if let Some(user_id) = calendar_get.user_public_id {
        Some(users::helpers::user_by_public_id(user_id, mm).await?)
    } else {
        None
    };

    let view = view! {
        <Calendar plan=plan user=user calendar_month=calendar_month/>
    }
    .to_html();
    Ok(Html(view))
}

// endregion: --- Calendar handler

// region:	  --- Date handler
::time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Deserialize)]
struct ToggleUserDate {
    #[serde(with = "date_format")]
    date: Date,
    user_public_id: Option<PublicId>,
}

// endregion: --- Date handler

#[component]
pub fn Calendar(
    plan: plans::Model,
    user: Option<users::Model>,
    calendar_month: CalendarMonth,
) -> impl IntoView {
    let calender_id = HtmxId::new("calender");

    view! {
        <div id=calender_id.clone().to_string() class="container mx-auto my-8">
            <SwitchMonthButton next_or_previous=SwitchMonth::Previous calendar_month=calendar_month calender_id=calender_id.clone()/>
            {calendar_month.month.to_string()} {calendar_month.year}
            <SwitchMonthButton next_or_previous=SwitchMonth::Next calendar_month=calendar_month calender_id=calender_id.clone() />
            <div class="grid grid-cols-7 gap-1 items-center justify-center">
                <Weekdays/>
                <div class="col-span-7 border-b-2 border-gray-400"></div>
                <Dates plan=plan user=user calendar_month=calendar_month/>
            </div>
        </div>
    }
}

/// A list of dates that are padded to fit a 7 day calendar
#[component]
fn Dates(
    /// The plan is used to collect other user's dates
    plan: plans::Model,
    /// The user currently editing the plan
    user: Option<users::Model>,
    /// Calendar month to be displayed
    calendar_month: CalendarMonth,
) -> impl IntoView {
    // TODO: implement the date handler and do client side stuff to immediately show clicked status
    // and disable submitting until response came back
    // --> Use Alpine JS here?

    calender_month_dates(calendar_month).into_iter().map(|date| {
        let user_public_id_include = HtmxInclude::from(htmx_ids::USER_PUBLIC_ID.clone()).to_string();

        view! {
            <button hx-post="calendar" name="date" value=date.to_string() hx-include=user_public_id_include type="submit"
                    class="ring-gray-400 hover:ring-1 h-12 {% if date.month() != calendar.month %}text-gray-500{% else %}text-white{% endif %}">
            </button>
        }
    }).collect_view()
}

#[component]
fn SwitchMonthButton(
    next_or_previous: SwitchMonth,
    calendar_month: CalendarMonth,
    calender_id: HtmxId,
) -> impl IntoView {
    let (switch_month_id, switch_year_id, switch_calendar_month, button_label) =
        match next_or_previous {
            SwitchMonth::Previous => (
                HtmxInput::new(HtmxId::new("previous_month"), "month"),
                HtmxInput::new(HtmxId::new("previous_year"), "year"),
                previous_month(calendar_month),
                "Previous",
            ),
            SwitchMonth::Next => (
                HtmxInput::new(HtmxId::new("next_month"), "month"),
                HtmxInput::new(HtmxId::new("next_year"), "year"),
                next_month(calendar_month),
                "Next",
            ),
        };

    let calendar_target = HtmxTarget::from(calender_id).to_string();
    let include_targets = HtmxInclude::from(vec![
        switch_month_id.clone(),
        switch_year_id.clone(),
        htmx_ids::USER_PUBLIC_ID.clone(),
    ])
    .to_string();

    view! {
        <HtmxHiddenInput input=switch_month_id value=switch_calendar_month.month/>
        <HtmxHiddenInput input=switch_year_id value=switch_calendar_month.year/>
        <button hx-get="calendar" hx-swap="outerHTML" hx-include=include_targets
                hx-target=calendar_target>{button_label}</button>
    }
}

const WEEKDAYS: [&str; 7] = ["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];
#[component]
fn Weekdays() -> impl IntoView {
    WEEKDAYS
        .into_iter()
        .map(|day| {
            view! {
                <div class="text-gray-400 font-bold">
                {day}
                </div>
            }
        })
        .collect_view()
}

enum SwitchMonth {
    Previous,
    Next,
}

// region:	  --- Utils
#[derive(Debug, Serialize, Copy, Clone)]
pub struct CalendarMonth {
    month: Month,
    year: i32,
}

impl CalendarMonth {
    pub fn current_month() -> Self {
        let now = OffsetDateTime::now_utc();

        Self {
            month: now.month(),
            year: now.year(),
        }
    }
}

fn previous_month(calendar_month: CalendarMonth) -> CalendarMonth {
    if calendar_month.month == Month::January {
        CalendarMonth {
            month: Month::December,
            year: calendar_month.year - 1,
        }
    } else {
        CalendarMonth {
            month: calendar_month.month.previous(),
            year: calendar_month.year,
        }
    }
}

fn next_month(calendar_month: CalendarMonth) -> CalendarMonth {
    if calendar_month.month == Month::December {
        CalendarMonth {
            month: Month::January,
            year: calendar_month.year + 1,
        }
    } else {
        CalendarMonth {
            month: calendar_month.month.next(),
            year: calendar_month.year,
        }
    }
}

// This adds the dates of the previous and next months until
// the first day is a monday and the last day a sunday
// for fitting on the calendar
fn calender_month_dates(calendar_month: CalendarMonth) -> Vec<Date> {
    let dates: Vec<Date> = (1..32)
        .map(|day| Date::from_calendar_date(calendar_month.year, calendar_month.month, day))
        .take_while(|date_result| date_result.is_ok())
        .filter_map(|date| date.ok()) // Basically just unwraps by throwing away errors, but errors have already been removed by the take_while
        .collect();

    let mut padded = vec![];

    // Pad dates backwards until the first day is Monday
    let mut first = *dates.first().unwrap();
    while first.weekday() != Weekday::Monday {
        if let Some(previous_day) = first.previous_day() {
            first = previous_day;
            padded.insert(0, first);
        } else {
            break;
        }
    }

    // Append the existing dates to the left padding
    padded.extend_from_slice(&dates);

    // Pad the dates forward until the last day is Sunday
    let mut last = *dates.last().unwrap();
    while last.weekday() != Weekday::Sunday {
        if let Some(next_day) = last.next_day() {
            last = next_day;
            padded.push(last);
        } else {
            break;
        }
    }

    padded
}

fn deserialize_month<'de, D>(deserializer: D) -> std::result::Result<Month, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "January" => Ok(Month::January),
        "February" => Ok(Month::February),
        "March" => Ok(Month::March),
        "April" => Ok(Month::April),
        "May" => Ok(Month::May),
        "June" => Ok(Month::June),
        "July" => Ok(Month::July),
        "August" => Ok(Month::August),
        "September" => Ok(Month::September),
        "October" => Ok(Month::October),
        "November" => Ok(Month::November),
        "December" => Ok(Month::December),
        _ => Err(serde::de::Error::custom(format!("Invalid month: {}", s))),
    }
}
// endregion: --- Utils

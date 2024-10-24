use std::collections::HashMap;

use axum::{
    debug_handler,
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use entity::{
    dates::{self},
    db::ModelManager,
    types::deserialize_public_id_option,
    types::PublicId,
    users,
};
use http::StatusCode;
use leptos::{either::Either, prelude::*};
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize};
use time::{Date, Month, OffsetDateTime, Weekday};
use tracing::debug;

use crate::{
    error::Result,
    htmx_helpers::{HtmxId, HtmxInclude, HtmxInput, HtmxTarget},
    plan_page::{filter_users_with_dates, htmx_ids},
    util_components::{HtmxHiddenInput, Icon},
};

use super::UserWithDates;

pub fn routes(mm: ModelManager) -> Router<entity::db::ModelManager> {
    Router::new().nest(
        "/calendar",
        Router::new()
            .route("/", get(get_calendar_handler))
            .route("/date", post(add_date_handler).delete(delete_date_handler))
            .with_state(mm),
    )
}

// region:	  --- Calendar handler

#[derive(Debug, Deserialize)]
struct CalendarGet {
    #[serde(deserialize_with = "deserialize_month")]
    month: Month,
    year: i32,
    #[serde(deserialize_with = "deserialize_public_id_option")]
    user_public_id: Option<PublicId>,
}

#[debug_handler]
async fn get_calendar_handler(
    State(mm): State<ModelManager>,
    Path(plan_public_id): Path<PublicId>,
    Query(calendar_get): Query<CalendarGet>,
) -> Result<impl IntoResponse> {
    debug!("{:<12} - calendar - {plan_public_id}", "HANDLER");

    // -- Calendar Month
    let calendar_month = CalendarMonth {
        month: calendar_get.month,
        year: calendar_get.year,
    };

    // -- Get the users and dates
    let users_with_dates =
        users::helpers::get_users_with_date_for_plan_public_id(plan_public_id, mm).await?;

    // -- Get the dates for the current user
    let current_user_with_dates = if let Some(user_public_id) = calendar_get.user_public_id {
        filter_users_with_dates(&users_with_dates, user_public_id)
    } else {
        None
    };

    let view = view! {
        <Calendar
            users_with_dates=users_with_dates
            current_user_with_dates=current_user_with_dates
            calendar_month=calendar_month
        />
    }
    .to_html();
    Ok(Html(view))
}

// endregion: --- Calendar handler

static CALENDAR_ID: Lazy<HtmxId> = Lazy::new(|| HtmxId::new("calendar"));
#[component]
pub fn Calendar(
    users_with_dates: Vec<UserWithDates>,
    current_user_with_dates: Option<UserWithDates>,
    calendar_month: CalendarMonth,
) -> impl IntoView {
    let calender_id = CALENDAR_ID.clone().to_string();

    view! {
        <div id=calender_id.clone() class="container mx-auto my-8">
            <div class="grid grid-cols-7 gap-1 items-center justify-center">
                <SwitchMonthButton
                    next_or_previous=SwitchMonth::Previous
                    calendar_month=calendar_month
                />
                <div class="text-lg col-span-5 font-bold text-center">
                    {calendar_month.month.to_string()} " " {calendar_month.year}
                </div>
                <SwitchMonthButton
                    next_or_previous=SwitchMonth::Next
                    calendar_month=calendar_month
                />
                <div class="col-span-7"></div>
                <Weekdays/>
                <div class="col-span-7 border-b-2 border-gray-400"></div>
                <Dates
                    users_with_dates=users_with_dates
                    current_user_with_dates=current_user_with_dates
                    calendar_month=calendar_month
                />
            </div>
        </div>
    }
}

// region:	  --- Date handlers
::time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
#[derive(Debug, Deserialize)]
struct ToggleDate {
    #[serde(with = "date_format")]
    date: Date,
    user_public_id: PublicId,
}

async fn add_date_handler(
    State(mm): State<ModelManager>,
    Path(plan_public_id): Path<PublicId>,
    Form(date_post): Form<ToggleDate>,
) -> Result<impl IntoResponse> {
    debug!(
        "{:<12} - add_date - {plan_public_id} - {}",
        "HANDLER", date_post.date
    );
    dates::helpers::user_add_date(date_post.user_public_id, date_post.date, mm).await?;

    Ok((StatusCode::CREATED).into_response())
}

async fn delete_date_handler(
    State(mm): State<ModelManager>,
    Path(plan_public_id): Path<PublicId>,
    Query(date_delete): Query<ToggleDate>,
) -> Result<impl IntoResponse> {
    debug!(
        "{:<12} - delete_date - {plan_public_id} - {}",
        "HANDLER", date_delete.date
    );

    dates::helpers::user_delete_date(date_delete.user_public_id, date_delete.date, mm).await?;

    Ok((StatusCode::OK).into_response())
}

// endregion: --- Date handlers

/// A list of dates that are padded to fit a 7 day calendar
#[component]
fn Dates(
    /// The user currently editing the plan
    current_user_with_dates: Option<UserWithDates>,
    /// A list of all users with their corresponding dates
    users_with_dates: Vec<UserWithDates>,
    /// Calendar month to be displayed
    calendar_month: CalendarMonth,
) -> impl IntoView {
    if let Some((user, dates)) = current_user_with_dates {
        let other_users = crate::plan_page::remove_user(users_with_dates, user.public_id);
        let ranked_dates = ranked_dates(&other_users);
        // Get the dates for the user
        Either::Left(
            calendar_month
                .dates()
                .into_iter()
                .map(|date| {
                    let others_date_rank = if let Some(rank) = ranked_dates.get(&date) {
                        *rank
                    } else {
                        DateRank::None
                    };
                    let selected = dates.iter().any(|date_model| date == date_model.date);

                    view! {
                        <InteractiveDate
                            date=date
                            calendar_month=calendar_month
                            selected=selected
                            others_date_rank=others_date_rank
                        />
                    }
                })
                .collect_view(),
        )
    } else {
        let ranked_dates = ranked_dates(&users_with_dates);
        Either::Right(
            calendar_month
                .dates()
                .into_iter()
                .map(|date| {
                    let date_rank = if let Some(rank) = ranked_dates.get(&date) {
                        *rank
                    } else {
                        DateRank::None
                    };

                    view! { <NonInteractiveDate date=date calendar_month=calendar_month date_rank=date_rank/> }
                })
                .collect_view(),
        )
    }
}

#[component]
fn NonInteractiveDate(
    date: Date,
    calendar_month: CalendarMonth,
    date_rank: DateRank,
) -> impl IntoView {
    let mut class = "relative h-12 w-full flex items-center justify-center".to_string();

    if is_today(date) {
        class += " ring-sky-500 ring-1"
    }

    if date.month() == calendar_month.month {
        class += " text-white"
    } else {
        class += " text-gray-500"
    }

    let mut selected_class = "absolute top-1/2 left-1/2  transform -translate-x-1/2 -translate-y-1/2  w-10 h-10 rounded-full z-0".to_string();

    match date_rank {
        DateRank::All => selected_class += " bg-slate-500",
        DateRank::Some => selected_class += " bg-slate-700",
        DateRank::None => selected_class = "".to_string(),
    }

    view! {
        <div class=class>
            <span class=selected_class></span>
            <span class="relative z-10">{date.day()}</span>
        </div>
    }
}

#[component]
fn InteractiveDate(
    date: Date,
    selected: bool,
    calendar_month: CalendarMonth,
    others_date_rank: DateRank,
) -> impl IntoView {
    let user_public_id = htmx_ids::USER_PUBLIC_ID.clone();
    let date_button_id = HtmxInput::new(HtmxId::new(&format!("date-{}", date)), "date");

    let include_targets =
        HtmxInclude::from(vec![user_public_id, date_button_id.clone()]).to_string();

    // TODO: Think of how to improve class composing in a less ad-hoc way
    let mut class = "relative h-12 w-full".to_string();

    if is_today(date) {
        class += " ring-sky-500 ring-1 hover:ring-gray-400"
    } else {
        class += " ring-gray-400 hover:ring-1"
    }

    if date.month() == calendar_month.month {
        class += " text-white"
    } else {
        class += " text-gray-500"
    }

    let mut selected_class = class.clone();

    match others_date_rank {
        DateRank::All => selected_class += " bg-slate-500",
        _ => selected_class += " bg-slate-700",
    }
    let xdata = if selected {
        "{isDelete : true}"
    } else {
        "{isDelete : false}"
    };

    let mut others_selected_class = "absolute top-1/2 left-1/2  transform -translate-x-1/2 -translate-y-1/2  w-12 h-12 rounded-full z-0".to_string();

    match others_date_rank {
        DateRank::All => others_selected_class += " bg-slate-500",
        DateRank::Some => others_selected_class += " bg-slate-700",
        DateRank::None => others_selected_class = "".to_string(),
    }

    view! {
        <div x-data=xdata>
            <HtmxHiddenInput input=date_button_id value=date/>
            <button
                x-show="!isDelete"
                hx-include=include_targets.clone()
                hx-swap="none"
                type="button"
                class=class
                x-on:click="isDelete = !isDelete"
                hx-post="calendar/date"
            >
                <span class=others_selected_class.clone()></span>
                <span class="relative z-10">{date.day()}</span>
            </button>
            <button
                x-show="isDelete"
                hx-include=include_targets
                hx-swap="none"
                type="button"
                class=selected_class
                x-on:click="isDelete = !isDelete"
                hx-delete="calendar/date"
            >
                <span class=others_selected_class></span>
                <span class="relative z-10">{date.day()}</span>
            </button>
        </div>
    }
}

enum SwitchMonth {
    Previous,
    Next,
}

#[component]
fn SwitchMonthButton(
    next_or_previous: SwitchMonth,
    calendar_month: CalendarMonth,
) -> impl IntoView {
    let (switch_month_id, switch_year_id, switch_calendar_month, button_icon) =
        match next_or_previous {
            SwitchMonth::Previous => (
                HtmxInput::new(HtmxId::new("previous_month"), "month"),
                HtmxInput::new(HtmxId::new("previous_year"), "year"),
                previous_month(calendar_month),
                Icon::Back,
            ),
            SwitchMonth::Next => (
                HtmxInput::new(HtmxId::new("next_month"), "month"),
                HtmxInput::new(HtmxId::new("next_year"), "year"),
                next_month(calendar_month),
                Icon::Forward,
            ),
        };

    let calendar_target = HtmxTarget::from(CALENDAR_ID.clone()).to_string();
    let include_targets = HtmxInclude::from(vec![
        switch_month_id.clone(),
        switch_year_id.clone(),
        htmx_ids::USER_PUBLIC_ID.clone(),
    ])
    .to_string();

    view! {
        <HtmxHiddenInput input=switch_month_id value=switch_calendar_month.month/>
        <HtmxHiddenInput input=switch_year_id value=switch_calendar_month.year/>
        <button
            hx-get="calendar"
            hx-swap="outerHTML"
            hx-include=include_targets
            hx-target=calendar_target
        >
            <div>
                <Icon icon=button_icon/>
            </div>
        </button>
    }
}

const WEEKDAYS: [&str; 7] = ["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];
#[component]
fn Weekdays() -> impl IntoView {
    WEEKDAYS
        .into_iter()
        .map(|day| {
            view! { <div class="text-gray-400 font-bold">{day}</div> }
        })
        .collect_view()
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

    // This adds the dates of the previous and next months until
    // the first day is a monday and the last day a sunday
    // for fitting on the calendar
    fn dates(&self) -> Vec<Date> {
        let dates: Vec<Date> = (1..32)
            .map(|day| Date::from_calendar_date(self.year, self.month, day))
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

fn is_today(date: Date) -> bool {
    let today = time::OffsetDateTime::now_utc().date();
    date == today
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateRank {
    All,
    Some,
    None,
}

pub fn ranked_dates(users_with_dates: &Vec<UserWithDates>) -> HashMap<Date, DateRank> {
    let total_users = users_with_dates.len();

    // Flatten dates and count
    let mut date_counts: HashMap<Date, usize> = HashMap::new();

    for (_, dates) in users_with_dates {
        for date in dates {
            *date_counts.entry(date.date).or_insert(0) += 1;
        }
    }

    // Step 2: Create a HashMap<Date, DateRank> to categorize each date
    let mut date_rankings: HashMap<Date, DateRank> = HashMap::new();

    for (date, count) in date_counts {
        let rank = if count == total_users {
            DateRank::All
        } else {
            DateRank::Some
        };

        date_rankings.insert(date, rank);
    }

    date_rankings
}
// endregion: --- Utils

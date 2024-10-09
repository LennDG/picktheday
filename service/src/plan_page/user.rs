use crate::{
    error::Result,
    htmx_helpers::{HtmxId, HtmxInclude, HtmxInput, HtmxTarget},
    plan_page::{
        calendar::{Calendar, CalendarMonth},
        htmx_ids,
    },
    util_components::{HtmxHiddenInput, HtmxSwapOob},
};
use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse, Response},
    routing::post,
    Form, Router,
};
use entity::{
    db::ModelManager,
    types::{PublicId, UserName},
    users::{self},
};
use http::StatusCode;
use leptos::prelude::*;
use once_cell::sync::Lazy;
use serde::Deserialize;
use tracing::debug;

use super::{filter_users_with_dates, UserWithDates};

pub fn routes(mm: ModelManager) -> Router<entity::db::ModelManager> {
    Router::new().nest(
        "/user",
        Router::new()
            .route("/", post(create_user_handler).get(change_user_handler))
            .with_state(mm),
    )
}

// region:	  --- User handlers
#[derive(Debug, Deserialize)]
struct UserPost {
    username: UserName,
}

#[derive(Debug)]
struct UpdateUserResponse {
    current_user_public_id: PublicId,
    users_with_dates: Vec<UserWithDates>,
}

impl IntoResponse for UpdateUserResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::OK;
        let current_user_with_dates =
            filter_users_with_dates(&self.users_with_dates, self.current_user_public_id.clone());
        let view = Html(
            view! {
                <UsersUpdate
                    users_with_dates=self.users_with_dates
                    current_user_public_id=self.current_user_public_id
                    current_user_with_dates
                />
            }
            .to_html(),
        );

        (status, view).into_response()
    }
}

async fn create_user_handler(
    State(mm): State<ModelManager>,
    Path(plan_public_id): Path<PublicId>,
    Form(user_post): Form<UserPost>,
) -> Result<impl IntoResponse> {
    debug!(
        "{:<12} - create_user_handler - {}",
        "HANDLER", user_post.username
    );

    // -- Create new user
    let new_user = users::helpers::create_user_for_plan(
        plan_public_id.clone(),
        user_post.username,
        mm.clone(),
    )
    .await?;

    //-- Get all users with their dates to use for result
    let users_with_dates =
        users::helpers::get_users_with_date_for_plan_public_id(plan_public_id, mm).await?;

    Ok(UpdateUserResponse {
        users_with_dates,
        current_user_public_id: new_user.public_id,
    }
    .into_response())
}

#[derive(Debug, Deserialize)]
struct UserGet {
    user_public_id: PublicId,
}

async fn change_user_handler(
    State(mm): State<ModelManager>,
    Path(plan_public_id): Path<PublicId>,
    Query(user_get): Query<UserGet>,
) -> Result<impl IntoResponse> {
    debug!(
        "{:<12} - change_user_handler - {}",
        "HANDLER", user_get.user_public_id
    );

    //-- Get all users with their dates to use for result
    let users_with_dates =
        users::helpers::get_users_with_date_for_plan_public_id(plan_public_id, mm).await?;

    Ok(UpdateUserResponse {
        users_with_dates,
        current_user_public_id: user_get.user_public_id,
    }
    .into_response())
}
// endregion: --- User handlers

#[component]
fn UsersUpdate(
    users_with_dates: Vec<UserWithDates>,
    current_user_public_id: PublicId,
    current_user_with_dates: Option<UserWithDates>,
) -> impl IntoView {
    let calendar_month = CalendarMonth::current_month();
    let calender_id = htmx_ids::CALENDAR_ID.clone();

    view! {
        <Users
            users_with_dates=users_with_dates.clone()
            current_user=Some(current_user_public_id.clone())
        />
        <HtmxSwapOob id=calender_id>
            <Calendar
                users_with_dates=users_with_dates
                calendar_month=calendar_month
                current_user_with_dates=current_user_with_dates
            />
        </HtmxSwapOob>
    }
}

static USERS_ID: Lazy<HtmxId> = Lazy::new(|| HtmxId::new("users"));
#[component]
pub fn Users(
    users_with_dates: Vec<UserWithDates>,
    current_user: Option<PublicId>,
) -> impl IntoView {
    let user_id = match current_user {
        Some(public_id) => public_id.to_string(),
        None => "".to_string(),
    };

    let users = users_with_dates
        .into_iter()
        .map(|user_dates| user_dates.0)
        .collect();

    view! {
        <div id="users">
            <HtmxHiddenInput input=htmx_ids::USER_PUBLIC_ID.clone() value=user_id/>
            <UserInput/>
            <UserList users=users/>
        </div>
    }
}

#[component]
fn UserInput() -> impl IntoView {
    view! {
        <div>
            <form
                hx-post="user"
                hx-target=HtmxTarget::from(USERS_ID.clone()).to_string()
                hx-swap="outerHTML"
                class="container relative z-0 mx-auto flex max-w-80 justify-center space-x-4"
            >
                <div>
                    <input
                        type="text"
                        id="username"
                        name="username"
                        class="border-1 peer block w-full appearance-none rounded-lg border border-gray-600 bg-transparent px-2 py-2.5 text-sm text-white outline-none focus:border-gray-300 "
                        placeholder="Your name"
                    />
                </div>
                <button
                    type="submit"
                    class="mb-2 me-2 flex rounded-lg border-gray-700 bg-gray-600 px-5 py-2.5 text-sm font-medium text-white hover:bg-gray-700"
                >
                    "Create"
                </button>

            </form>
        </div>
    }
}

#[component]
fn UserList(users: Vec<users::Model>) -> impl IntoView {
    view! {
        <ul class="mx-auto max-w-80 mt-4 space-y-2">
            {users
                .into_iter()
                .map(|user| {
                    let username = user.name.to_string();
                    let input = HtmxInput::new(
                        HtmxId::new(&format!("user{}", &user.public_id)),
                        "user_public_id",
                    );
                    let include = HtmxInclude::from(input.clone()).to_string();
                    let target = HtmxTarget::from(USERS_ID.clone()).to_string();
                    view! {
                        <li class="flex justify-between items-center border-b border-gray-700 py-2">
                            <HtmxHiddenInput input=input value=user.public_id/>
                            <span class="text-white">{username}</span>
                            <button
                                hx-get="user"
                                hx-target=target
                                hx-include=include
                                class="p-2 text-gray-400 hover:text-white"
                            >
                                "Edit"
                            </button>
                        </li>
                    }
                })
                .collect_view()}

        </ul>
    }
}

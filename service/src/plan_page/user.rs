use crate::{
    error::Result,
    htmx_helpers::{HtmxId, HtmxTarget},
    plan_page::{
        calendar::{Calendar, CalendarMonth},
        htmx_ids,
    },
    util_components::{HtmxHiddenInput, HtmxSwapOob},
};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Response},
    routing::post,
    Form, Router,
};
use entity::{
    db::ModelManager,
    plans::{self},
    sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter},
    types::{PublicId, UserName},
    users::{self, NewUser},
};
use http::StatusCode;
use leptos::prelude::*;
use serde::Deserialize;
use tracing::debug;

use super::UsersWithDates;

pub fn routes(mm: ModelManager) -> Router<entity::db::ModelManager> {
    Router::new().nest(
        "/user",
        Router::new()
            .route("/", post(create_user_handler))
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
    users_with_dates: UsersWithDates,
}

impl IntoResponse for UpdateUserResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::CREATED;
        let view = Html(
            view! {
                <UsersUpdate
                    users_with_dates=self.users_with_dates
                    current_user_public_id=self.current_user_public_id
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
// endregion: --- User handlers

#[component]
fn UsersUpdate(
    users_with_dates: UsersWithDates,
    current_user_public_id: PublicId,
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
                current_user=Some(current_user_public_id)
            />
        </HtmxSwapOob>
    }
}

#[component]
pub fn Users(users_with_dates: UsersWithDates, current_user: Option<PublicId>) -> impl IntoView {
    let user_id = match current_user {
        Some(public_id) => public_id.to_string(),
        None => "".to_string(),
    };

    let users_id = HtmxId::new("users");
    let users = users_with_dates
        .into_iter()
        .map(|user_dates| user_dates.0)
        .collect();

    view! {
        <div id="users">
            <HtmxHiddenInput input=htmx_ids::USER_PUBLIC_ID.clone() value=user_id />
            <UserInput users_id=users_id />
            <UserList users=users />
        </div>
    }
}

#[component]
fn UserInput(users_id: HtmxId) -> impl IntoView {
    view! {
        <form
            hx-post="user"
            hx-target=HtmxTarget::from(users_id).to_string()
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
                <button
                    type="submit"
                    class="mb-2 me-2 flex rounded-lg border-gray-700 bg-gray-600 px-5 py-2.5 text-sm font-medium text-white hover:bg-gray-700"
                >
                    "Create"
                </button>
            </div>
        </form>
    }
}

#[component]
fn UserList(users: Vec<users::Model>) -> impl IntoView {
    users
        .into_iter()
        .map(|user| {
            let username = user.name;
            view! { <ul>{username.to_string()}</ul> }
        })
        .collect_view()
}

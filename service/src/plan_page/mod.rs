use crate::{app::Page, error::Result};
use axum::{
    body::Body,
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Form, Router,
};
use axum_htmx::HxRedirect;
use calendar::{Calendar, CalendarMonth};
use entity::{
    dates,
    db::ModelManager,
    plans::{self},
    sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter},
    types::{PlanName, PublicId},
    users,
};
use http::{StatusCode, Uri};
use leptos::prelude::*;
use serde::Deserialize;
use tracing::debug;
use user::Users;

mod calendar;
mod htmx_ids;
mod user;

pub fn routes(mm: ModelManager) -> Router {
    Router::new().nest(
        "/plan",
        Router::new()
            .route("/", post(create_plan_handler))
            .route("/:plan_slug", get(redirect_plan_handler))
            .nest(
                "/:plan_slug/",
                Router::new()
                    .route("/", get(plan_page_handler))
                    .merge(calendar::routes(mm.clone()))
                    .merge(user::routes(mm.clone())),
            )
            .with_state(mm),
    )
}

// region:	  --- Plan creation
#[derive(Debug, Deserialize)]
struct PlanPost {
    plan_name: PlanName,
}

#[derive(Debug)]
struct CreatePlanResponse {
    plan_url: Uri,
}

impl IntoResponse for CreatePlanResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::CREATED;
        let redirect = HxRedirect::from(self.plan_url);
        let body = Body::empty();

        (status, redirect, body).into_response()
    }
}

async fn create_plan_handler(
    State(mm): State<ModelManager>,
    Form(plan_post): Form<PlanPost>,
) -> Result<CreatePlanResponse> {
    debug!(
        "{:<12} - create_plan_handler - {}",
        "HANDLER", plan_post.plan_name
    );

    let new_plan = plans::helpers::create_plan(plan_post.plan_name, mm).await?;

    let plan_url = format!("/plan/{}/", new_plan.public_id).parse::<Uri>()?;

    // Return an empty body with the HX-Redirect header
    Ok(CreatePlanResponse { plan_url })
}
// endregion: --- Plan creation

// region:	  --- Plan page
async fn plan_page_handler(
    State(mm): State<ModelManager>,
    Path(plan_public_id): Path<PublicId>,
) -> Result<impl IntoResponse> {
    debug!("{:<12} - plan_page_handler - {plan_public_id}", "HANDLER");

    // -- Get the plan
    let plan = plans::helpers::plan_by_public_id(plan_public_id.clone(), mm.clone()).await?;

    // -- Get the users with dates
    let users_with_dates =
        users::helpers::get_users_with_date_for_plan_public_id(plan_public_id, mm).await;

    let view = view! { <PlanPage plan=plan users_with_dates=users_with_dates.unwrap()/> }.to_html();
    Ok(Html(view))
}

#[component]
fn PlanPage(plan: plans::Model, users_with_dates: Vec<UserWithDates>) -> impl IntoView {
    let plan_title = plan.name.to_string();
    view! {
        <Page title=plan_title.clone()>
            <div>
                <h1>{plan_title}</h1>
            </div>

            <Calendar
                users_with_dates=users_with_dates.clone()
                current_user_with_dates=None
                calendar_month=CalendarMonth::current_month()
            />
            <Users users_with_dates=users_with_dates current_user=None/>
        </Page>
    }
}
// endregion: --- Plan page

// region:	  --- Plan Redirect

// TODO: Check if I can't do this with a Replace URL

// It's important that the the url ends in a `/` otherwise the routes don't work
async fn redirect_plan_handler(Path(page_slug): Path<String>) -> impl IntoResponse {
    //let plan_url = format!("/plan/{}/", page_slug).parse::<Uri>()?;
    let uri = &format!("/plan/{}/", page_slug);
    Redirect::permanent(uri)
}
// endregion: --- Plan Redirect

// region:	  --- Utilities

pub type UserWithDates = (users::Model, Vec<dates::Model>);

fn filter_users_with_dates(
    users_with_dates: &Vec<UserWithDates>,
    user_public_id: PublicId,
) -> Option<UserWithDates> {
    // -- Get the dates for the current user
    users_with_dates
        .iter()
        .find(|user_with_dates| user_with_dates.0.public_id == user_public_id)
        .cloned()
}

// impl UsersWithDates {}
// endregion: --- Utilities

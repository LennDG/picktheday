use crate::{
    app::{NotFound, Page},
    error::{Error, Result},
};
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
    db::ModelManager,
    plans::{self, NewPlan},
    sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter},
    types::{PlanName, PublicId},
    users,
};
use http::{StatusCode, Uri};
use leptos::prelude::*;
use serde::Deserialize;
use tracing::{debug, info};
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

impl TryFrom<PlanPost> for NewPlan {
    type Error = Error;

    fn try_from(plan_post: PlanPost) -> Result<Self> {
        Ok(NewPlan::new(plan_post.plan_name, None))
    }
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

    let new_plan = NewPlan::try_from(plan_post)?;
    let new_plan_entity = new_plan.into_active_model().insert(mm.db()).await?;

    let plan_url = format!("/plan/{}/", new_plan_entity.public_id).parse::<Uri>()?;

    // Return an empty body with the HX-Redirect header
    Ok(CreatePlanResponse { plan_url })
}
// endregion: --- Plan creation

// region:	  --- Plan page
async fn plan_page_handler(
    State(mm): State<ModelManager>,
    Path(plan_public_id): Path<PublicId>,
) -> Result<impl IntoResponse> {
    info!("{:<12} - plan_page_handler - {plan_public_id}", "HANDLER");

    // -- Get the plan
    let plan = plans::helpers::plan_by_public_id(plan_public_id, mm.clone()).await?;

    // -- Get the users
    let users = plan.get_users(mm).await?;

    let view = view! { <PlanPage plan=plan users=users /> }.to_html();
    Ok(Html(view))
}

#[component]
fn PlanPage(plan: plans::Model, users: Vec<users::Model>) -> impl IntoView {
    let plan_title = plan.name.to_string();

    view! {
        <Page title=plan_title.clone()>
            <div>
                <h1>{plan_title}</h1>
            </div>
            <Calendar plan=plan.clone() user=None calendar_month=CalendarMonth::current_month() />
            <Users users=users current_user=None />
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

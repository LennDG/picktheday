use crate::{
    app::{NotFound, Page},
    error::{Error, Result},
};
use axum::{
    body::Body,
    extract::{Path, State},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use axum_htmx::HxRedirect;
use entity::{
    db::ModelManager,
    plans::{self, NewPlan},
    sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter},
    types::PlanName,
};
use http::{StatusCode, Uri};
use leptos::prelude::*;
use serde::Deserialize;
use tracing::debug;
use user::Users;

mod calendar;
mod user;

pub fn routes(mm: ModelManager) -> Router {
    Router::new().nest(
        "/plan",
        Router::new()
            .route("/", post(create_plan_handler))
            .nest(
                "/:plan_slug",
                Router::new()
                    .route("/", get(plan_page_handler))
                    .merge(calendar::routes(mm.clone())),
            )
            .with_state(mm),
    )
}

// region:	  --- Plan creation
#[derive(Debug, Deserialize)]
struct PlanPost {
    new_plan: String,
}

impl TryFrom<PlanPost> for NewPlan {
    type Error = Error;

    fn try_from(plan_post: PlanPost) -> Result<Self> {
        let plan_name = PlanName::new(&plan_post.new_plan)
            .map_err(|err| Error::NewPlanInvalid(err.to_string()))?;
        Ok(NewPlan::new(plan_name, None))
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
        "HANDLER", plan_post.new_plan
    );

    // -- Check if string is not empty
    if plan_post.new_plan.is_empty() {
        return Err(Error::NewPlanInvalid("Name empty".to_string()));
    }

    let new_plan = NewPlan::try_from(plan_post)?;
    let new_plan_entity = new_plan.into_active_model().insert(mm.db()).await?;

    let plan_url = format!("/plan/{}", new_plan_entity.public_id).parse::<Uri>()?;

    // Return an empty body with the HX-Redirect header
    Ok(CreatePlanResponse { plan_url })
}
// endregion: --- Plan creation

// region:	  --- Plan page
async fn plan_page_handler(
    State(mm): State<ModelManager>,
    Path(page_slug): Path<String>,
) -> Result<impl IntoResponse> {
    // -- Get the page
    let plan = plans::Entity::find()
        .filter(plans::Column::PublicId.eq(page_slug))
        .one(mm.db())
        .await?;

    if let Some(plan) = plan {
        Ok(Html(view! {<PlanPage plan=plan/>}.to_html()))
    } else {
        Ok(Html(NotFound().to_html()))
    }
}

#[component]
fn PlanPage(plan: plans::Model) -> impl IntoView {
    let plan_title = plan.name.to_string();

    view! {
        <Page title=plan_title.clone()>
            <div>
                <h1>{plan_title}</h1>
            </div>
            //<Calendar/>
            <Users plan=plan current_user=None/>
        </Page>
    }
}

// endregion: --- Plan page

use crate::{
    error::{Error, Result},
    plan_page::htmx_ids,
    util_components::HtmxHiddenInput,
};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    routing::post,
    Form, Router,
};
use entity::{
    db::ModelManager,
    plans::{self},
    sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter},
    types::{PublicId, UserName},
    users::NewUser,
};
use http::StatusCode;
use leptos::prelude::*;
use serde::Deserialize;
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router<entity::db::ModelManager> {
    Router::new()
        .route("/user", post(create_user_handler))
        .with_state(mm)
}

// region:	  --- User handlers
#[derive(Debug, Deserialize)]
struct UserPost {
    new_user: String,
}

async fn create_user_handler(
    State(mm): State<ModelManager>,
    Path(plan_public_id): Path<String>,
    Form(user_post): Form<UserPost>,
) -> Result<Response> {
    debug!(
        "{:<12} - create_user_handler - {}",
        "HANDLER", user_post.new_user
    );

    // -- Check if user name not empty
    if user_post.new_user.is_empty() {
        return Err(Error::NewUserInvalid("Name empty".to_string()));
    }

    // -- Get the plan
    let plan = plans::Entity::find()
        .filter(plans::Column::PublicId.eq(plan_public_id))
        .one(mm.db())
        .await?;

    // -- Create user if the plan exists
    if let Some(plan_model) = plan {
        let user_name = UserName::new(&user_post.new_user)
            .map_err(|err| Error::NewUserInvalid(err.to_string()))?;
        let new_user = NewUser::new(user_name, plan_model.id);

        // TODO: Give clear error to user when username already exists
        let new_user_entity = new_user.into_active_model().insert(mm.db()).await?;

        return Ok((StatusCode::CREATED).into_response());
    } else {
        return Ok((StatusCode::NOT_FOUND).into_response());
    }
}
// endregion: --- User handlers

#[component]
pub fn Users(plan: plans::Model, current_user: Option<PublicId>) -> impl IntoView {
    let user_id = match current_user {
        Some(public_id) => public_id.to_string(),
        None => "".to_string(),
    };

    view! {
        <div>
            <HtmxHiddenInput input=htmx_ids::USER_PUBLIC_ID.clone() value=user_id/>
            <div>"Placeholder"</div>
        </div>
    }
}

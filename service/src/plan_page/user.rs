use crate::{
    error::{Error, Result},
    htmx_helpers::{HtmxId, HtmxTarget},
    plan_page::htmx_ids,
    util_components::HtmxHiddenInput,
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
    sea_orm::{
        ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter,
    },
    types::{PublicId, UserName},
    users::{self, NewUser},
};
use http::StatusCode;
use leptos::prelude::*;
use serde::Deserialize;
use tracing::debug;

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
struct CreateUserResponse {
    users: Vec<users::Model>,
    current_user: PublicId,
}

impl IntoResponse for CreateUserResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::CREATED;
        let view = Html(
            view! { <Users users=self.users current_user=Some(self.current_user) /> }
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

    // -- Get the plan
    let plan = plans::Entity::find()
        .filter(plans::Column::PublicId.eq(plan_public_id))
        .one(mm.db())
        .await?;

    // -- Create user if the plan exists
    if let Some(plan_model) = plan {
        let new_user = NewUser::new(user_post.username, plan_model.id);

        // TODO: Give clear error to user when username already exists
        let new_user_model = new_user.into_active_model().insert(mm.clone().db()).await?;

        //-- Get all users
        let users = plan_model.get_users(mm).await?;

        Ok(CreateUserResponse {
            users,
            current_user: new_user_model.public_id,
        }
        .into_response())
    } else {
        Ok((StatusCode::NOT_FOUND).into_response())
    }
}
// endregion: --- User handlers

#[component]
pub fn Users(users: Vec<users::Model>, current_user: Option<PublicId>) -> impl IntoView {
    let user_id = match current_user {
        Some(public_id) => public_id.to_string(),
        None => "".to_string(),
    };

    let users_id = HtmxId::new("users");

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

// use crate::leptos_axum::LeptosHtml;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use entity::db::ModelManager;

use http::StatusCode;
use leptos::prelude::*;
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new().route("/", get(index_page)).with_state(mm)
}

pub async fn index_page() -> Html<String> {
    debug!("{:<12} - index", "HANDLER");

    let content = view! {
        <Page title="Pick The Day!".to_string()>
            <HomePage/>
        </Page>
    }
    .to_html();

    Html(content)
}

pub async fn not_found_page() -> impl IntoResponse {
    let content = view! {
        <Page title="Pick The Day!".to_string()>
            <NotFound/>
        </Page>
    }
    .to_html();

    (StatusCode::NOT_FOUND, Html(content)).into_response()
}

#[component]
pub fn Page(title: String, children: Children) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <title>{title}</title>

                <StyleSheetLink/>

                <script src="https://unpkg.com/htmx.org@2.0.2/dist/htmx.min.js" defer></script>
                <script src="https://unpkg.com/alpinejs@3.14.1/dist/cdn.min.js" defer></script>
                // <AlpineGlobalState/>

                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <meta charset="utf-8"/>

                <script>let FF_FOUC_FIX;</script>
            </head>

            <body class="container relative mx-auto bg-slate-800">
                <main class="container text-white text-center pt-16">{children()}</main>
            </body>
        </html>
    }
}

#[component]
fn StyleSheetLink() -> impl IntoView {
    view! {
        <link
            rel="preload"
            href="/main.css"
            type="text/css"
            r#as="style"
            onload="this.onload=null;this.rel='stylesheet'"
        />
    }
}

#[component]
fn AlpineGlobalState() -> impl IntoView {
    view! { <script>"document.addEventListener('alpine:init', () => {Alpine.store({})})"</script> }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div>
            <h1>"Pick The Day"</h1>
            <p>"Create a meetup!"</p>

            <form
                hx-post="/plan"
                class="container relative z-0 mx-auto flex max-w-80 justify-center space-x-4"
            >
                <div>
                    <input
                        type="text"
                        name="plan_name"
                        class="border-1 peer block w-full appearance-none rounded-lg border border-gray-600 bg-transparent px-2 py-2.5 text-sm text-white outline-none focus:border-gray-300 "
                        placeholder="e.g. Tennis"
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
pub fn NotFound() -> impl IntoView {
    view! {
        <div>
            <h1>"Not Found"</h1>
            <p>"Page not found"</p>
            <a href="/">"Back to Home"</a>
        </div>
    }
}

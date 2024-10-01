// use crate::leptos_axum::LeptosHtml;
use axum::{response::Html, routing::get, Router};
use entity::db::ModelManager;

use leptos::prelude::*;
use leptos_meta::Meta;
use tracing::info;

pub fn routes(mm: ModelManager) -> Router {
    Router::new().route("/", get(index_page)).with_state(mm)
}

pub async fn index_page() -> Html<String> {
    info!("{:<12} - index", "HANDLER");

    let content = view! {
        <Page title="Pick The Day!".to_string()>
            <HomePage/>
        </Page>
    }
    .to_html();

    Html(content)
}

pub async fn not_found_page() -> Html<String> {
    let content = view! {
        <Page title="Pick The Day!".to_string()>
            <NotFound/>
        </Page>
    }
    .to_html();

    Html(content)
}

#[component]
pub fn Page(title: String, children: Children) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <title>{title}</title>

                <StyleSheetLink/>
                // <link rel="stylesheet" type="text/css" href="/main.css"/>

                <script src="https://unpkg.com/htmx.org@1.9.2/dist/htmx.min.js" defer/>
                <script src="https://unpkg.com/alpinejs@3.14.1/dist/cdn.min.js" defer/>
                //<AlpineGlobalState/>

                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <meta charset="utf-8"/>
            </head>

            <body class= "container relative mx-auto bg-slate-800">
                <main class="container text-white text-center pt-16">
                    {children()}
                </main>
            </body>
        </html>
    }
}

#[component]
fn StyleSheetLink() -> impl IntoView {
    view! { <link rel="preload" href="/main.css" type="text/css" r#as="style" onload="this.rel='stylesheet'" /> }
}

#[component]
fn AlpineGlobalState() -> impl IntoView {
    view! {
        <script>
            "document.addEventListener('alpine:init', () => {Alpine.store({})})"
        </script>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div>
            <h1>"Pick The Day"</h1>
            <p>"Create a meetup!"</p>

            <form class="container relative z-0 mx-auto flex max-w-80 justify-center space-x-4">
                <div>
                    <input type="text" id="new_plan" name="new_plan"
                        class="border-1 peer block w-full appearance-none rounded-lg border border-gray-600 bg-transparent px-2 py-2.5 text-sm text-white outline-none focus:border-gray-300 "
                        placeholder="e.g. Tennis" />
                </div>
                <button type="submit" hx-post="/plan" hx-include="#new_plan"
                    class="mb-2 me-2 flex rounded-lg border-gray-700 bg-gray-600 px-5 py-2.5 text-sm font-medium text-white hover:bg-gray-700">"Create"</button>
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

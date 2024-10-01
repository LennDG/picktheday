use axum::response::Response as AxumResponse;
use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
};
use tower::ServiceExt;
use tower_http::services::ServeDir;

use crate::app::not_found_page;
use crate::config::web_config;

pub async fn file_and_error_handler(req: Request<Body>) -> AxumResponse {
    let root = &web_config().WEB_FOLDER;
    let (parts, _) = req.into_parts();

    let mut static_parts = parts.clone();
    static_parts.headers.clear();
    if let Some(encodings) = parts.headers.get("accept-encoding") {
        static_parts
            .headers
            .insert("accept-encoding", encodings.clone());
    }

    let res = get_static_file(Request::from_parts(static_parts, Body::empty()), root)
        .await
        .unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        not_found_page().await.into_response()
    }
}

async fn get_static_file(
    request: Request<Body>,
    root: &str,
) -> Result<Response<Body>, (StatusCode, String)> {
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root)
        .precompressed_gzip()
        .precompressed_br()
        .oneshot(request)
        .await
    {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error serving files: {err}"),
        )),
    }
}

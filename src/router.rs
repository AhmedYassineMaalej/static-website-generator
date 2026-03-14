use axum::response::Html;
use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    http::{HeaderMap, HeaderValue, header},
    response::IntoResponse,
    routing::get,
};


use crate::app_state::ProjectState;

async fn root(State(state): State<Arc<ProjectState>>) -> impl IntoResponse {
    Html(state.template.get_html().await)
}

async fn script(State(state): State<Arc<ProjectState>>) -> impl IntoResponse {
    let javascript = state.template.get_javascript().await;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/javascript"),
    );

    (headers, javascript)
}

async fn css(State(state): State<Arc<ProjectState>>) -> impl IntoResponse {
    state.template.get_css().await
}

pub fn get_router(state: Arc<ProjectState>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/script.js", get(script))
        .route("/styles.css", get(css))
        .with_state(state)
}

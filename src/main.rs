mod app_state;
mod connection;
mod file_contents;
mod watcher;

use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue, header};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tracing::info;

use crate::app_state::AppState;
use crate::connection::Connection;
use crate::file_contents::ProjectFiles;
use crate::watcher::FileWatcher;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let md_path = PathBuf::from("input.md").canonicalize().unwrap();
    let html_path = PathBuf::from("public/index.html").canonicalize().unwrap();
    let css_path = PathBuf::from("public/styles.css").canonicalize().unwrap();
    let js_path = PathBuf::from("public/script.js").canonicalize().unwrap();

    let (update_sender, _) = broadcast::channel(10);

    let files = Arc::new(
        ProjectFiles::new(html_path, md_path, css_path, js_path, update_sender.clone()).await,
    );

    let state = Arc::new(AppState::new(files.clone(), update_sender.clone()).await);

    let watcher = FileWatcher::new();

    info!("file watcher is ready");

    tokio::spawn(watcher.watch(state.clone()));
    info!("started watching files");

    let server = TcpListener::bind("127.0.0.1:9001").await.unwrap();

    let state_ref = state.clone();
    tokio::spawn(async move {
        while let Ok((stream, addr)) = server.accept().await {
            let update_receiver = update_sender.subscribe();
            let connection =
                Connection::new(stream, addr, state_ref.clone(), update_receiver).await;

            tokio::spawn(connection.handle());
        }
    });

    let server_thread = tokio::spawn(async {
        let app = Router::new()
            .route("/", get(root))
            .route("/script.js", get(script))
            .route("/styles.css", get(css))
            .with_state(state);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        info!("server listening on http://localhost:3000");
        axum::serve(listener, app).await.unwrap();
    });

    let mut child = tokio::process::Command::new("firefox")
        .arg("http://localhost:3000")
        .spawn()
        .unwrap();

    child.wait().await.unwrap();

    server_thread.await;
}

#[derive(Debug, Serialize)]
pub struct Update {
    #[serde(rename = "type")]
    ttype: UpdateEvent,
    payload: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum UpdateEvent {
    Html,
    Css,
}

async fn root(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Html(fs::read_to_string("public/index.html").await.unwrap())
}

async fn script(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let javascript = state.files.javascript.get_content().await;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/javascript"),
    );

    (headers, javascript)
}

async fn css(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    state.files.css.get_content().await.clone()
}

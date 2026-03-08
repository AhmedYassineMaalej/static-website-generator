#![warn(clippy::pedantic)]

mod app_state;
mod article;
mod article_state;
mod config;
mod connection;
mod file_contents;
mod highlight;
mod html_generator;
mod index_extractor;
mod server;
mod text_extractor;
mod visitor;
mod watcher;

use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue, header};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use futures_util::StreamExt;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{RwLock, broadcast};
use tokio_tungstenite::accept_async;
use tracing::info;

use crate::app_state::ProjectState;
use crate::article_state::ArticleState;
use crate::config::{Config, ProjectDirectories};
use crate::connection::{ListenerConnection, UpdaterConnection};
use crate::server::Server;
use crate::watcher::FileWatcher;

const ARTICLES_DIR: &str = "article/";
const TEMPLATE_DIR: &str = "template/";
const PUBLIC_DIR: &str = "public/";

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let config = Config::new(ProjectDirectories::new(
        &PathBuf::from(ARTICLES_DIR),
        &PathBuf::from(PUBLIC_DIR),
        &PathBuf::from(TEMPLATE_DIR),
    ));

    let (update_sender, _rx) = broadcast::channel(10);

    let watcher = FileWatcher::new(config.directories.clone());
    info!("file watcher is ready");

    let state = Arc::new(ProjectState::new(config).await);
    let server = Server::new(state.clone(), update_sender.clone());
    let (_server_task, server_handle) = server.run();

    tokio::spawn(watcher.watch(server_handle.clone()));
    info!("started watching files");

    let cloned_state = state.clone();
    let server_thread = tokio::spawn(async {
        let app = Router::new()
            .route("/", get(root))
            .route("/script.js", get(script))
            .route("/styles.css", get(css))
            .with_state(cloned_state);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        info!("server listening on http://localhost:3000");
        axum::serve(listener, app).await.unwrap();
    });

    tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();
        while let Ok((stream, addr)) = listener.accept().await {
            let websocket = accept_async(stream).await.unwrap();
            let (ws_write, ws_read) = websocket.split();

            let updater = UpdaterConnection::new(
                ws_write,
                addr,
                update_sender.subscribe(),
                server_handle.clone(),
            );
            let listener = ListenerConnection::new(ws_read);

            let _updater_task = updater.run();
            let _listener_task = listener.run();
        }
    });

    let mut child = tokio::process::Command::new("xdg-open")
        .arg("http://localhost:3000")
        .spawn()
        .unwrap();

    child.wait().await.unwrap();

    server_thread.await.unwrap();
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Update {
    Markdown { content: String, index: String },
    Html, // dont need to send anything as page will just reload
    Css { css: String },
    Javascript,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum UpdateEvent {
    Markdown,
    Html,
    Css,
    Javascript,
}

#[derive(Clone, Debug)]
pub enum ClientEvent {
    SendArticle(Arc<RwLock<ArticleState>>),
    SendCss(String),
    Reload,
}

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

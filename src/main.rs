#![warn(clippy::pedantic)]

mod app_state;
mod article;
mod article_state;
mod compile;
mod config;
mod connection;
mod extractor;
mod highlight;
mod html_generator;
mod image_extractors;
mod index_extractor;
mod markdown_node;
mod markdown_visitor;
mod metadata_extractor;
pub mod router;
mod server;
mod text_extractor;
mod watcher;

use axum::Router;
use clap::{Parser, Subcommand};
use futures_util::StreamExt;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tracing::info;

use crate::app_state::{ProjectState, Template};
use crate::compile::compile_articles;
use crate::config::WatcherConfig;
use crate::connection::{ListenerConnection, UpdaterConnection};
use crate::router::get_router;
use crate::server::{ConnectionEvent, NewConnectionEvent, Server, ServerEvent};
use crate::watcher::FileWatcher;

const ARTICLES_DIR: &str = "articles/";
const TEMPLATE_DIR: &str = "template/";
const OUTPUT_DIR: &str = "public/";

const HTTP_ADDR: &str = "0.0.0.0:3000";
const WEBSOCKET_ADDR: &str = "127.0.0.1:9001";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Builds the articles into target directory
    Build,
    /// Launches server for hot reloading
    Dev {
        /// Path to the article to preview
        article: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let article = match cli.command {
        Commands::Build => {
            let template = Template::from_directory(&PathBuf::from(TEMPLATE_DIR)).await;
            compile_articles(
                &PathBuf::from(ARTICLES_DIR),
                &PathBuf::from(OUTPUT_DIR),
                &template,
            )
            .await;

            return;
        }
        Commands::Dev { article } => article,
    };

    let subscriber = tracing_subscriber::fmt().with_line_number(true).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let config = WatcherConfig::new(&article, &PathBuf::from(TEMPLATE_DIR));

    let watcher = FileWatcher::new(config.clone());
    info!("file watcher is ready");

    let state = Arc::new(ProjectState::new(config).await);
    let server = Server::new(state.clone());
    let (_server_task, server_handle) = server.run();

    tokio::spawn(watcher.watch(server_handle.clone()));
    info!("started watching files");

    let cloned_state = state.clone();
    let http_server_task = tokio::spawn(async {
        let app = Router::new().merge(get_router(cloned_state));

        let listener = tokio::net::TcpListener::bind(HTTP_ADDR).await.unwrap();
        info!("server listening on http://localhost:3000");
        axum::serve(listener, app).await.unwrap();
    });

    tokio::spawn(async move {
        let listener = TcpListener::bind(WEBSOCKET_ADDR).await.unwrap();
        while let Ok((stream, addr)) = listener.accept().await {
            let websocket = accept_async(stream).await.unwrap();
            let (ws_write, ws_read) = websocket.split();
            let updater = UpdaterConnection::new(ws_write, addr);
            let listener = ListenerConnection::new(ws_read, addr, server_handle.clone());

            let (_updater_task, updater_handle) = updater.run();
            let _listener_taks = listener.run();

            server_handle
                .send(ServerEvent::ConnectionEvent(ConnectionEvent::Open(
                    NewConnectionEvent {
                        sender: updater_handle,
                        addr,
                    },
                )))
                .unwrap();
        }
    });

    let mut child = tokio::process::Command::new("xdg-open")
        .arg("http://localhost:3000")
        .spawn()
        .unwrap();

    child.wait().await.unwrap();

    http_server_task.await.unwrap();
}

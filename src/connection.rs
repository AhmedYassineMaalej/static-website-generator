use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use serde::Serialize;
use tokio::{
    net::TcpStream,
    sync::{
        RwLock, broadcast,
        mpsc::{UnboundedSender, unbounded_channel},
    },
    task::JoinHandle,
};
use tokio_tungstenite::{
    WebSocketStream,
    tungstenite::{Error, Message},
};
use tracing::{info, warn};

use crate::{
    article_state::ArticleState,
    server::{ConnectionEvent, NewConnectionEvent, ServerEvent},
};

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Update {
    Markdown {
        content: String,
        index: String,
        title: String,
        tags: String,
    },
    Html, // dont need to send anything as page will just reload
    Css {
        css: String,
    },
}

/// events that are _sent_ to the user
#[derive(Clone, Debug)]
pub enum ClientEvent {
    SendArticle(Arc<RwLock<ArticleState>>),
    SendCss(String),
    Reload,
}

pub struct UpdaterConnection {
    socket: SplitSink<WebSocketStream<TcpStream>, Message>,
    addr: SocketAddr,
}

pub struct ListenerConnection {
    socket: SplitStream<WebSocketStream<TcpStream>>,
    server: UnboundedSender<ServerEvent>,
    addr: SocketAddr,
}

impl ListenerConnection {
    pub fn new(
        socket: SplitStream<WebSocketStream<TcpStream>>,
        addr: SocketAddr,
        server: UnboundedSender<ServerEvent>,
    ) -> Self {
        Self {
            socket,
            server,
            addr,
        }
    }

    pub fn run(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(Ok(msg)) = self.socket.next().await {
                if let Message::Close(_) = msg {
                    info!("client closed connection");
                    self.server
                        .send(ServerEvent::ConnectionEvent(ConnectionEvent::Close(
                            self.addr,
                        )))
                        .unwrap();

                    continue;
                }

                if let Message::Text(text) = msg {
                    let text = text.to_string();
                    let Some((line, offset)) = text.split_once(':') else {
                        warn!("received invalid text: {text}");
                        continue;
                    };

                    // nvim --server /tmp/nv.sock --remote-send '<ESC>50G10|'
                    let arg = format!("<ESC>{line}G{offset}|");
                    let mut command = tokio::process::Command::new("nvim");
                    command.args(["--server", "/tmp/nv.sock", "--remote-send", arg.as_str()]);

                    let Ok(mut child) = command.spawn() else {
                        warn!("failed to execute nvim command");
                        continue;
                    };

                    child.wait().await.unwrap();
                }
            }
        })
    }
}

impl UpdaterConnection {
    pub fn new(socket: SplitSink<WebSocketStream<TcpStream>, Message>, addr: SocketAddr) -> Self {
        Self { socket, addr }
    }

    pub async fn send_event(&mut self, event: ClientEvent) -> Result<(), Error> {
        let update = match event {
            ClientEvent::SendArticle(article) => {
                let lock = article.read().await;
                Update::Markdown {
                    content: lock.content_html.clone(),
                    index: lock.index_html.clone(),
                    title: lock.article.title_html(),
                    tags: lock.article.tags_html(),
                }
            }
            ClientEvent::SendCss(css) => Update::Css { css },
            ClientEvent::Reload => Update::Html,
        };

        self.socket
            .send(Message::from(serde_json::to_string(&update).unwrap()))
            .await
    }

    #[must_use]
    pub fn run(mut self) -> (JoinHandle<()>, UnboundedSender<ClientEvent>) {
        let (event_tx, mut event_rx) = unbounded_channel();

        let event_tx2 = event_tx.clone();
        let task = tokio::spawn(async move {
            info!("client connected on {}", self.addr);

            while let Some(event) = event_rx.recv().await {
                let content_type = match event {
                    ClientEvent::SendArticle(_) => "article",
                    ClientEvent::SendCss(_) => "css",
                    ClientEvent::Reload => "html",
                };

                let res = self.send_event(event).await;
                match res {
                    Ok(()) => {
                        info!("sent updated {}", content_type);
                    }
                    Err(Error::ConnectionClosed) => {
                        info!("closed connection to {}", self.addr);
                        break;
                    }
                    Err(e) => {
                        info!("error in sending: {e:?}");
                    }
                }
            }
        });

        (task, event_tx)
    }
}

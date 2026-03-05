use std::{net::SocketAddr, sync::Arc};

use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use tokio::{
    net::TcpStream,
    sync::{RwLock, broadcast},
};
use tokio_tungstenite::{
    WebSocketStream, accept_async,
    tungstenite::{Error, Message, Utf8Bytes},
};
use tracing::info;

use crate::{
    ClientEvent, Update, UpdateEvent,
    app_state::{ProjectState, Template},
    article::Article,
    article_state::ArticleState,
};

pub struct Connection {
    ws_read: SplitStream<WebSocketStream<TcpStream>>,
    ws_write: SplitSink<WebSocketStream<TcpStream>, Message>,
    addr: SocketAddr,
    state: Arc<ProjectState>,
    event_rx: broadcast::Receiver<ClientEvent>,
}

impl Connection {
    pub async fn new(
        stream: TcpStream,
        addr: SocketAddr,
        state: Arc<ProjectState>,
        event_rx: broadcast::Receiver<ClientEvent>,
    ) -> Self {
        let websocket = accept_async(stream).await.unwrap();
        let (ws_write, ws_read) = websocket.split();

        Self {
            addr,
            state,
            event_rx,
            ws_write,
            ws_read,
        }
    }

    pub async fn send_html(&mut self) -> Result<(), Error> {
        let article = self.state.article.read().await.html.clone();
        let update = Update {
            ttype: UpdateEvent::Html,
            payload: article,
        };

        self.ws_write
            .send(Message::from(serde_json::to_string(&update).unwrap()))
            .await?;

        info!("sent html to {}", self.addr);
        Ok(())
    }

    pub async fn send_css(&mut self) -> Result<(), Error> {
        let css = self.state.template.get_css().await;
        let update = Update {
            ttype: UpdateEvent::Css,
            payload: css,
        };

        self.ws_write
            .send(Message::from(serde_json::to_string(&update).unwrap()))
            .await?;

        info!("sent css to {}", self.addr);
        Ok(())
    }

    pub async fn handle(mut self) {
        info!("client connected on {}", self.addr);
        self.send_html().await.unwrap();
        self.send_css().await.unwrap();

        let Self {
            mut ws_read,
            mut ws_write,
            addr,
            mut event_rx,
            state,
        } = self;

        tokio::spawn(async move {
            while let Some(Ok(msg)) = ws_read.next().await {
                info!("received message: {msg:?}");
                if let Message::Text(text) = msg {
                    let text = text.to_string();
                    let (line, offset) = text.split_once(':').unwrap();
                    // nvim --server /tmp/nv.sock --remote-send '<ESC>50G10|'
                    let arg = format!("<ESC>{line}G{offset}|");
                    let mut command = tokio::process::Command::new("nvim");
                    command.args(["--server", "/tmp/nv.sock", "--remote-send", arg.as_str()]);

                    let mut child = command.spawn().unwrap();
                    child.wait().await.unwrap();
                }
            }
        });

        while let Ok(event) = event_rx.recv().await {
            let res = match event {
                ClientEvent::ReloadArticle => {
                    let html = state.article.read().await.html.clone();
                    let update = Update {
                        ttype: UpdateEvent::Html,
                        payload: html,
                    };
                    match ws_write
                        .send(Message::from(serde_json::to_string(&update).unwrap()))
                        .await
                    {
                        Ok(()) => {
                            info!("sent html to {}", addr);
                            Ok(())
                        }
                        Err(e) => Err(e),
                    }
                }
                ClientEvent::ReloadCss => {
                    let css = state.template.get_css().await;
                    let update = Update {
                        ttype: UpdateEvent::Css,
                        payload: css,
                    };

                    match ws_write
                        .send(Message::from(serde_json::to_string(&update).unwrap()))
                        .await
                    {
                        Ok(()) => {
                            info!("sent css to {}", addr);
                            Ok(())
                        }
                        Err(e) => Err(e),
                    }
                }
            };

            if let Err(Error::ConnectionClosed) = res {
                info!("closed connected to {}", self.addr);
                break;
            }
        }
    }
}

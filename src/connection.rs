use std::{net::SocketAddr, sync::Arc};

use futures_util::SinkExt;
use tokio::{net::TcpStream, sync::broadcast};
use tokio_tungstenite::{
    WebSocketStream, accept_async,
    tungstenite::{Error, Message},
};
use tracing::info;

use crate::{Update, UpdateEvent, app_state::AppState, file_contents::ProjectFiles};

pub struct Connection {
    websocket: WebSocketStream<TcpStream>,
    addr: SocketAddr,
    state: Arc<AppState>,
    event_rx: broadcast::Receiver<UpdateEvent>,
}

impl Connection {
    pub async fn new(
        stream: TcpStream,
        addr: SocketAddr,
        state: Arc<AppState>,
        event_rx: broadcast::Receiver<UpdateEvent>,
    ) -> Self {
        let websocket = accept_async(stream).await.unwrap();

        Self {
            addr,
            state,
            event_rx,
            websocket,
        }
    }

    pub async fn send_html(&mut self) -> Result<(), Error> {
        let html = self.state.html.read().await;
        let update = Update {
            ttype: UpdateEvent::Html,
            payload: html.to_string(),
        };

        self.websocket
            .send(Message::from(serde_json::to_string(&update).unwrap()))
            .await?;

        info!("sent html to {}", self.addr);
        Ok(())
    }

    pub async fn send_css(&mut self) -> Result<(), Error> {
        let css = self.state.files.css.get_content().await;
        let update = Update {
            ttype: UpdateEvent::Css,
            payload: css,
        };

        self.websocket
            .send(Message::from(serde_json::to_string(&update).unwrap()))
            .await?;

        info!("sent css to {}", self.addr);
        Ok(())
    }

    pub async fn handle(mut self) {
        info!("client connected on {}", self.addr);
        self.send_html().await.unwrap();
        self.send_css().await.unwrap();

        while let Ok(update_type) = self.event_rx.recv().await {
            let res = match update_type {
                UpdateEvent::Html => self.send_html().await,
                UpdateEvent::Css => self.send_css().await,
            };

            if let Err(Error::ConnectionClosed) = res {
                self.disconnect().await;
                break;
            }
        }
    }

    async fn disconnect(mut self) {
        self.websocket.close(None).await.unwrap();
        info!("closed connected to {}", self.addr);
    }
}

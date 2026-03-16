use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use tokio::{
    sync::mpsc::{UnboundedSender, unbounded_channel},
    task,
};

use crate::app_state::ProjectState;
use crate::config::UpdateEvent;
use crate::connection::ClientEvent;

pub struct Server {
    state: Arc<ProjectState>,
    broadcast_list: HashMap<SocketAddr, UnboundedSender<ClientEvent>>,
}

pub enum ServerEvent {
    ConnectionEvent(ConnectionEvent),
    Update(UpdateEvent),
}

pub enum ConnectionEvent {
    Open(NewConnectionEvent),
    Close(SocketAddr),
}

pub struct NewConnectionEvent {
    pub addr: SocketAddr,
    pub sender: UnboundedSender<ClientEvent>,
}

impl Server {
    pub fn new(state: Arc<ProjectState>) -> Self {
        Self {
            state,
            broadcast_list: HashMap::new(),
        }
    }

    pub fn run(mut self) -> (task::JoinHandle<()>, UnboundedSender<ServerEvent>) {
        let (handle, mut receiver) = unbounded_channel();
        let task = tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                self.handle_event(event).await;
            }
        });

        (task, handle)
    }

    async fn handle_event(&mut self, event: ServerEvent) {
        match event {
            ServerEvent::ConnectionEvent(conn_event) => self.handle_connection(conn_event).await,
            ServerEvent::Update(update_event) => self.handle_update(update_event).await,
        }
    }

    async fn handle_update(&self, event: UpdateEvent) {
        match event {
            UpdateEvent::Markdown => {
                self.state.update_markdown().await;
                self.broadcast_article();
            }
            UpdateEvent::Html => {
                self.broadcast(&ClientEvent::Reload);
            }
            UpdateEvent::Css => {
                self.broadcast_css().await;
            }
            UpdateEvent::Javascript => {
                // nothing for now
            }
        }
    }

    async fn handle_connection(&mut self, event: ConnectionEvent) {
        match event {
            ConnectionEvent::Open(connection) => {
                let article = self.state.article.clone();
                let css = self.get_css().await;

                connection
                    .sender
                    .send(ClientEvent::SendArticle(article))
                    .unwrap();

                connection.sender.send(ClientEvent::SendCss(css)).unwrap();

                self.broadcast_list
                    .insert(connection.addr, connection.sender);
                // self.broadcast_article();
                // self.broadcast_css().await;
            }
            ConnectionEvent::Close(ip_addr) => {
                self.broadcast_list.remove(&ip_addr);
            }
        }
    }

    fn broadcast(&self, message: &ClientEvent) {
        for connection in self.broadcast_list.values() {
            connection
                .send(message.clone())
                .expect("failed to send message");
        }
    }

    async fn broadcast_css(&self) {
        let css = self.get_css().await;
        self.broadcast(&ClientEvent::SendCss(css));
    }

    fn broadcast_article(&self) {
        let article = self.state.article.clone();
        self.broadcast(&ClientEvent::SendArticle(article));
    }

    async fn get_css(&self) -> String {
        let css = self.state.template.get_css().await;
        css + "
span.word:hover {
    color: darkorange;
}
"
    }
}

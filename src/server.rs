use std::sync::Arc;

use tokio::{
    sync::{
        broadcast,
        mpsc::{UnboundedSender, unbounded_channel},
    },
    task,
};

use crate::app_state::ProjectState;
use crate::config::UpdateEvent;
use crate::connection::ClientEvent;

pub struct Server {
    state: Arc<ProjectState>,
    broadcast_sender: broadcast::Sender<ClientEvent>,
}

pub enum ServerEvent {
    ConnectionEvent(ConnectionEvent),
    Update(UpdateEvent),
}

pub enum ConnectionEvent {
    NewConnection,
}

impl Server {
    pub fn new(state: Arc<ProjectState>, broadcast_sender: broadcast::Sender<ClientEvent>) -> Self {
        Self {
            state,
            broadcast_sender,
        }
    }

    pub fn run(self) -> (task::JoinHandle<()>, UnboundedSender<ServerEvent>) {
        let (handle, mut receiver) = unbounded_channel();
        let task = tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                self.handle_event(event).await;
            }
        });

        (task, handle)
    }

    async fn handle_event(&self, event: ServerEvent) {
        match event {
            ServerEvent::ConnectionEvent(conn_event) => self.handle_connection(conn_event).await,
            ServerEvent::Update(update_event) => self.handle_update(update_event).await,
        }
    }

    async fn handle_update(&self, event: UpdateEvent) {
        match event {
            UpdateEvent::Markdown => {
                self.state.update_markdown().await;
                self.broadcoast_article();
            }
            UpdateEvent::Html => {
                self.broadcoast_html();
            }
            UpdateEvent::Css => {
                self.broadcoast_css().await;
            }
            UpdateEvent::Javascript => {
                // nothing for now
            }
        }
    }

    async fn handle_connection(&self, event: ConnectionEvent) {
        match event {
            ConnectionEvent::NewConnection => {
                self.broadcoast_article();
                self.broadcoast_css().await;
            }
        }
    }

    fn broadcoast_html(&self) {
        self.broadcast_sender
            .send(ClientEvent::Reload)
            .expect("failed to send html");
    }

    async fn broadcoast_css(&self) {
        let css = self.state.template.get_css().await;
        let css = css
            + "
span.word:hover {
    color: darkorange;
}
";
        self.broadcast_sender
            .send(ClientEvent::SendCss(css))
            .expect("failed to send css");
    }

    fn broadcoast_article(&self) {
        let article = self.state.article.clone();
        self.broadcast_sender
            .send(ClientEvent::SendArticle(article))
            .expect("failed to send article");
    }
}

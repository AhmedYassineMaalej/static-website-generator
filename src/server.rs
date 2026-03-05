use std::sync::Arc;

use tokio::{
    sync::{
        broadcast,
        mpsc::{UnboundedSender, unbounded_channel},
    },
    task,
};

use crate::{ClientEvent, UpdateEvent, app_state::ProjectState};

pub struct Server {
    state: Arc<ProjectState>,
    broadcast_sender: broadcast::Sender<ClientEvent>,
}

pub enum ServerEvent {
    ConnectionEvent,
    Update(UpdateEvent),
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
            ServerEvent::ConnectionEvent => todo!(),
            ServerEvent::Update(update_event) => self.handle_update(update_event).await,
        }
    }

    async fn handle_update(&self, event: UpdateEvent) {
        match event {
            UpdateEvent::Markdown => {
                self.state.update_markdown().await;
                self.broadcast_sender
                    .send(ClientEvent::ReloadArticle)
                    .unwrap();
            }
            UpdateEvent::Html => todo!(),
            UpdateEvent::Css => {
                //  add anything here to be done before sending css
                self.broadcast_sender.send(ClientEvent::ReloadCss).unwrap();
            }
            UpdateEvent::Javascript => todo!(),
        }
    }
}

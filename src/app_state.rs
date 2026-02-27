use std::{path::Path, sync::Arc};

use tokio::sync::{RwLock, broadcast, mpsc::UnboundedSender};
use tracing::info;

use crate::{UpdateEvent, file_contents::ProjectFiles};

pub struct AppState {
    pub files: Arc<ProjectFiles>,
    pub html: RwLock<String>,
    pub update_sender: broadcast::Sender<UpdateEvent>,
}

impl AppState {
    pub async fn new(
        files: Arc<ProjectFiles>,
        update_sender: broadcast::Sender<UpdateEvent>,
    ) -> Self {
        let html = markdown::to_html(&files.markdown.get_content().await);

        Self {
            html: RwLock::new(html),
            files,
            update_sender,
        }
    }

    pub async fn handle_update(&self, path: &Path) {
        if path == &self.files.markdown.path {
            info!("html changed");
            self.files.markdown.update().await;
            let markdown = self.files.markdown.get_content().await;
            let html = markdown::to_html(&markdown);
            let mut lock = self.html.write().await;
            *lock = html;
            self.update_sender.send(UpdateEvent::Html);
        } else if path == &self.files.javascript.path {
            info!("javascript changed");
            self.files.javascript.update().await;
        } else if path == &self.files.css.path {
            info!("css changed");
            self.files.css.update().await;
            self.update_sender.send(UpdateEvent::Css);
        }
    }
}

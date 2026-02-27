use std::{
    ffi::OsStr,
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};

use tokio::{
    fs,
    sync::{RwLock, RwLockReadGuard, broadcast},
};

use tracing::info;

use crate::UpdateEvent;

pub struct ProjectFiles {
    pub html: ProjectFile,
    pub markdown: ProjectFile,
    pub css: ProjectFile,
    pub javascript: ProjectFile,
    update_sender: broadcast::Sender<UpdateEvent>,
}

pub struct ProjectFile {
    pub path: PathBuf,
    content: RwLock<String>,
    update_sender: broadcast::Sender<UpdateEvent>,
}

impl ProjectFile {
    pub async fn new(path: PathBuf, update_sender: broadcast::Sender<UpdateEvent>) -> Self {
        Self {
            content: RwLock::new(fs::read_to_string(&path).await.unwrap()),
            path,
            update_sender,
        }
    }

    pub async fn update(&self) {
        *self.content.write().await = fs::read_to_string(&self.path).await.unwrap();
    }

    pub async fn get_content(&self) -> String {
        let lock = self.content.read().await;
        lock.clone()
    }
}

impl ProjectFiles {
    pub async fn new(
        html: PathBuf,
        markdown: PathBuf,
        css: PathBuf,
        javascript: PathBuf,
        update_sender: broadcast::Sender<UpdateEvent>,
    ) -> Self {
        Self {
            markdown: ProjectFile::new(markdown, update_sender.clone()).await,
            html: ProjectFile::new(html, update_sender.clone()).await,
            css: ProjectFile::new(css, update_sender.clone()).await,
            javascript: ProjectFile::new(javascript, update_sender.clone()).await,
            update_sender,
        }
    }
}

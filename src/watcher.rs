use std::{path::Path, sync::Arc};

use notify::{
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher, event::ModifyKind,
};

use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver},
};
use tracing::info;

use crate::{UpdateEvent, app_state::AppState, file_contents::ProjectFiles};

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    file_event_recv: UnboundedReceiver<Result<Event, notify::Error>>,
}

impl FileWatcher {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                tx.send(res).unwrap();
            },
            Config::default(),
        )
        .unwrap();

        watcher
            .watch(Path::new("."), RecursiveMode::Recursive)
            .unwrap();

        FileWatcher {
            watcher,
            file_event_recv: rx,
        }
    }

    pub async fn watch(mut self, state: Arc<AppState>) {
        while let Some(Ok(event)) = self.file_event_recv.recv().await {
            let Event { kind, paths, .. } = event;

            let EventKind::Modify(ModifyKind::Data(_change)) = kind else {
                continue;
            };

            for path in paths.into_iter().flat_map(|p| p.canonicalize()) {
                state.handle_update(&path).await;
            }
        }
    }
}

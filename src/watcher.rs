use notify::{
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher, event::ModifyKind,
};

use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{config::ProjectDirectories, server::ServerEvent};

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    file_event_recv: UnboundedReceiver<Result<Event, notify::Error>>,
    directories: ProjectDirectories,
}

impl FileWatcher {
    pub fn new(dirs: ProjectDirectories) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                tx.send(res).unwrap();
            },
            Config::default(),
        )
        .unwrap();

        watcher
            .watch(&dirs.public, RecursiveMode::NonRecursive)
            .unwrap();

        watcher
            .watch(&dirs.article, RecursiveMode::Recursive)
            .unwrap();

        watcher
            .watch(&dirs.template, RecursiveMode::NonRecursive)
            .unwrap();

        FileWatcher {
            watcher,
            file_event_recv: rx,
            directories: dirs,
        }
    }

    pub async fn watch(mut self, server_handle: UnboundedSender<ServerEvent>) {
        while let Some(Ok(event)) = self.file_event_recv.recv().await {
            let Event { kind, paths, .. } = event;

            let EventKind::Modify(ModifyKind::Data(_change)) = kind else {
                continue;
            };

            for path in paths.into_iter().flat_map(|p| p.canonicalize()) {
                if let Some(update) = self.directories.process_change(&path) {
                    server_handle.send(ServerEvent::Update(update)).unwrap();
                }
            }
        }
    }
}

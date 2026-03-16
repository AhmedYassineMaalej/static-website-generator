use notify::{
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher, event::ModifyKind,
};

use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tracing::info;

use crate::{config::WatcherConfig, server::ServerEvent};

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    file_event_recv: UnboundedReceiver<Result<Event, notify::Error>>,
    config: WatcherConfig,
}

impl FileWatcher {
    pub fn new(watcher_config: WatcherConfig) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                tx.send(res).unwrap();
            },
            Config::default(),
        )
        .unwrap();

        // watch article parent directory instead of file
        // this is because some editors, on save, delete the file and create
        // a new file with the same name
        let article_directory = watcher_config.article.parent().unwrap();

        watcher
            .watch(article_directory, RecursiveMode::Recursive)
            .unwrap();

        watcher
            .watch(&watcher_config.template, RecursiveMode::NonRecursive)
            .unwrap();

        FileWatcher {
            watcher,
            file_event_recv: rx,
            config: watcher_config,
        }
    }

    pub async fn watch(mut self, server_handle: UnboundedSender<ServerEvent>) {
        while let Some(Ok(event)) = self.file_event_recv.recv().await {
            let Event { kind, paths, .. } = event;

            let EventKind::Modify(ModifyKind::Data(_change)) = kind else {
                continue;
            };

            for path in paths.into_iter().flat_map(|p| p.canonicalize()) {
                info!("change detected");
                if let Some(update) = self.config.process_change(&path) {
                    server_handle.send(ServerEvent::Update(update)).unwrap();
                }
            }
        }
    }
}

use std::path::{Path, PathBuf};

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum UpdateEvent {
    Markdown,
    Html,
    Css,
    Javascript,
}

#[derive(Clone)]
pub struct WatcherConfig {
    pub article: PathBuf,
    pub template: PathBuf,
}

impl WatcherConfig {
    pub fn new(article: &Path, template: &Path) -> Self {
        Self {
            article: article.canonicalize().unwrap(),
            template: template.canonicalize().unwrap(),
        }
    }

    pub fn process_change(&self, path: &Path) -> Option<UpdateEvent> {
        let path = path.canonicalize().unwrap();

        if path == self.article {
            return Some(UpdateEvent::Markdown);
        }

        if path.starts_with(&self.template) {
            let extension = path.extension().unwrap();
            if extension == "css" {
                return Some(UpdateEvent::Css);
            }

            if extension == "js" {
                return Some(UpdateEvent::Javascript);
            }

            if extension == "html" {
                return Some(UpdateEvent::Html);
            }
        }

        None
    }
}

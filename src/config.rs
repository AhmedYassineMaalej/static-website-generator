use std::path::{Path, PathBuf};

use crate::UpdateEvent;

#[derive(Clone)]
pub struct Config {
    pub directories: ProjectDirectories,
}

impl Config {
    pub fn new(directories: ProjectDirectories) -> Self {
        Self { directories }
    }
}

#[derive(Clone)]
pub struct ProjectDirectories {
    pub article: PathBuf,
    pub public: PathBuf,
    pub template: PathBuf,
}

impl ProjectDirectories {
    pub fn new(
        article_directory: &Path,
        public_directory: &Path,
        template_directory: &Path,
    ) -> Self {
        Self {
            article: article_directory.canonicalize().unwrap(),
            public: public_directory.canonicalize().unwrap(),
            template: template_directory.canonicalize().unwrap(),
        }
    }

    pub fn process_change(&self, path: &Path) -> Option<UpdateEvent> {
        let path = path.canonicalize().unwrap();

        if path.starts_with(&self.article) {
            return Some(UpdateEvent::Markdown);
        }

        if path.starts_with(&self.template) {
            if path.extension().unwrap() == "css" {
                return Some(UpdateEvent::Css);
            }

            if path.extension().unwrap() == "js" {
                return Some(UpdateEvent::Html);
            }

            if path.extension().unwrap() == "html" {
                return Some(UpdateEvent::Javascript);
            }
        }

        None
    }
}

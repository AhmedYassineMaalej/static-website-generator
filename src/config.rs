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
    pub article_directory: PathBuf,
    pub public_directory: PathBuf,
    pub template_directory: PathBuf,
}

impl ProjectDirectories {
    pub fn new(
        article_directory: PathBuf,
        public_directory: PathBuf,
        template_directory: PathBuf,
    ) -> Self {
        Self {
            article_directory: article_directory.canonicalize().unwrap(),
            public_directory: public_directory.canonicalize().unwrap(),
            template_directory: template_directory.canonicalize().unwrap(),
        }
    }

    pub fn process_change(&self, path: &Path) -> Option<UpdateEvent> {
        let path = path.canonicalize().unwrap();

        if path.starts_with(&self.article_directory) {
            return Some(UpdateEvent::Markdown);
        }

        if path.starts_with(&self.template_directory) {
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

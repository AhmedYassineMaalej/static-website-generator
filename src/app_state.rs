use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use markdown::ParseOptions;
use tokio::{
    fs,
    sync::{
        RwLock, broadcast,
        mpsc::{UnboundedSender, unbounded_channel},
    },
};
use tracing::info;

use crate::{
    ClientEvent, UpdateEvent, article::Article, article_state::ArticleState, config::Config,
};

pub struct ProjectState {
    pub config: Config,
    pub template: Arc<Template>,
    pub article: Arc<RwLock<ArticleState>>,
}

pub struct Template {
    html: PathBuf,
    css: PathBuf,
    javascript: PathBuf,
}

impl Template {
    pub fn new(html: PathBuf, css: PathBuf, javascript: PathBuf) -> Self {
        Self {
            html,
            css,
            javascript,
        }
    }

    pub async fn from_directory(path: &Path) -> Self {
        let mut dir = tokio::fs::read_dir(path).await.unwrap();

        let mut html = None;
        let mut css = None;
        let mut javascript = None;

        while let Ok(Some(file)) = dir.next_entry().await {
            let path = file.path();
            let Some(extension) = path.extension() else {
                continue;
            };

            if extension == "html" {
                html = Some(path);
                continue;
            }

            if extension == "css" {
                css = Some(path);
                continue;
            }

            if extension == "js" {
                javascript = Some(path);
                continue;
            }
        }

        Self::new(
            html.expect("no html template found"),
            css.expect("no css template found"),
            javascript.expect("no javascript template found"),
        )
    }

    pub async fn get_html(&self) -> String {
        fs::read_to_string(&self.html).await.unwrap()
    }

    pub async fn get_css(&self) -> String {
        fs::read_to_string(&self.css).await.unwrap()
    }

    pub async fn get_javascript(&self) -> String {
        fs::read_to_string(&self.javascript).await.unwrap()
    }
}

impl ProjectState {
    pub async fn new(config: Config) -> Self {
        let template = Template::from_directory(&config.directories.template_directory).await;
        let mut article_directory = tokio::fs::read_dir(&config.directories.article_directory)
            .await
            .unwrap();

        let mut article = None;
        while let Ok(Some(article_entry)) = article_directory.next_entry().await {
            let article_dir_path = article_entry.path();
            let article_state = ArticleState::from_file(&article_dir_path).await;
            article = Some(article_state);
        }

        let Some(article) = article else {
            panic!("no article found");
        };

        Self {
            config,
            template: Arc::new(template),
            article: Arc::new(RwLock::new(article)),
        }
    }

    pub async fn update_markdown(&self) {
        self.article.write().await.update().await;
    }
}

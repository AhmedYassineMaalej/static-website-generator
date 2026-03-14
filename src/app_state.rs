use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use tokio::{fs, sync::RwLock};

use crate::{article_state::ArticleState, config::WatcherConfig};

pub struct ProjectState {
    pub config: WatcherConfig,
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

            panic!("unknown file: {}", path.display());
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
    pub async fn new(config: WatcherConfig) -> Self {
        let template = Template::from_directory(&config.template).await;
        let article = ArticleState::from_file(&config.article).await;

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

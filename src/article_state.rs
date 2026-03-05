use std::path::{Path, PathBuf};

use markdown::ParseOptions;
use tokio::fs;

use crate::article::Article;

pub struct ArticleState {
    pub path: PathBuf,
    pub article: Article,
    pub html: String,
}

impl ArticleState {
    pub fn new(article: Article, path: PathBuf) -> Self {
        Self {
            html: article.html(),
            path,
            article,
        }
    }

    pub async fn from_file(path: &Path) -> Self {
        let markdown = fs::read_to_string(&path).await.unwrap();

        let mdast = markdown::to_mdast(&markdown, &ParseOptions::default()).unwrap();
        let article = Article::new(mdast);
        Self::new(article, path.to_path_buf())
    }

    pub async fn update(&mut self) {
        let markdown = fs::read_to_string(&self.path).await.unwrap();
        let mdast = markdown::to_mdast(&markdown, &ParseOptions::default()).unwrap();
        self.article = Article::new(mdast);
        self.html = self.article.html();
    }
}

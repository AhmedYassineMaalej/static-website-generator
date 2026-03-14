use std::path::{Path, PathBuf};

use markdown::ParseOptions;
use tokio::fs;

use crate::article::Article;

#[derive(Debug)]
pub struct ArticleState {
    pub path: PathBuf,
    pub article: Article,
    pub content_html: String,
    pub index_html: String,
}

impl ArticleState {
    pub fn new(article: Article, path: PathBuf) -> Self {
        Self {
            content_html: article.content_html(true),
            index_html: article.index_html(),
            path,
            article,
        }
    }

    pub async fn from_file(path: &Path) -> Self {
        let markdown = fs::read_to_string(&path).await.unwrap();

        let options = Self::parse_options();

        let mdast = markdown::to_mdast(&markdown, &options).unwrap();
        let article = Article::new(mdast);
        Self::new(article, path.to_path_buf())
    }

    pub async fn update(&mut self) {
        let markdown = fs::read_to_string(&self.path).await.unwrap();

        let options = Self::parse_options();

        let mdast = markdown::to_mdast(&markdown, &options).unwrap();

        self.article = Article::new(mdast);
        self.content_html = self.article.content_html(true);
        self.index_html = self.article.index_html();
    }

    fn parse_options() -> ParseOptions {
        let mut options = ParseOptions::default();
        options.constructs.frontmatter = true;
        options.constructs.math_flow = true;

        options
    }
}

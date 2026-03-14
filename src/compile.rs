use std::path::Path;

use tokio::fs;

use crate::{
    app_state::Template, article::Article, article_state::ArticleState,
    image_extractors::ArticleImage,
};

async fn compile_article(article: &Article, template: &Template) -> String {
    let article_html = article.content_html(false);
    let index_html = article.index_html();
    let css = template.get_css().await;
    let css_html = format!("<style>{css}</style>");
    let title = &article.metadata.title;
    let title_html = format!("<h1 class=\"title\">{title}</h1>");

    format!(
        r#"
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <script
      defer
      src="https://cdn.jsdelivr.net/npm/mathjax@4/tex-mml-chtml.js"
    ></script>
    {css_html}
    <title>{title}</title>
  </head>
  <body>
    <div class="title-container">
    {title_html}
    </div>
    <div class="tags-container">
    </div>
    <div class="content-container">
{article_html}
    </div>
    <div class="index-container">
    {index_html}
    </div>
  </body>
  <script src="script.js"></script>
</html>
"#
    )
}

pub async fn compile_articles(
    articles_directory: &Path,
    output_directory: &Path,
    template: &Template,
) {
    let mut articles = fs::read_dir(articles_directory).await.unwrap();

    while let Ok(Some(article)) = articles.next_entry().await {
        let path = article.path();
        let state = ArticleState::from_file(&path).await;

        let post_dir = output_directory.join(path.file_stem().unwrap());

        // clear and create article directory
        if post_dir.exists() {
            fs::remove_dir_all(&post_dir).await;
        }

        fs::create_dir(&post_dir).await.unwrap();

        // add html
        let html = compile_article(&state.article, template).await;
        fs::write(post_dir.join("index.html"), html).await.unwrap();

        // move images
        for image in &state.article.images {
            if let ArticleImage::Local(image_path) = image {
                fs::copy(image_path, post_dir.join(image_path.file_name().unwrap()))
                    .await
                    .unwrap();
            }
        }
    }
}

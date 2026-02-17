use crate::html::{HTMLNode, ToHtml};

pub struct Renderer;

impl Renderer {
    pub fn render_page(htmlnode: HTMLNode) -> String {
        let body = htmlnode.to_html();
        format!(
            "
<!DOCTYPE html>
<html>
    <head>
        <title>Rendered Markdown</title>
    </head>
    {body}
</html>
",
        )
    }
}


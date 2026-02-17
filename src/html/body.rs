use crate::html::ToHtml;

use super::HTMLNode;

pub struct Body {
    pub(crate) children: Vec<HTMLNode>,
}

impl ToHtml for Body {
    fn to_html(&self) -> String {
        format!("<body>{}</body>", self.children.to_html())
    }
}

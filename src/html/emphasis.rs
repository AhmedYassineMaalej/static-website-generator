use crate::html::ToHtml;

use super::HTMLNode;

pub struct Emphasis {
    pub children: Vec<HTMLNode>,
}

impl ToHtml for Emphasis {
    fn to_html(&self) -> String {
        let children = self.children.to_html();
        format!("<i>{children}</i>")
    }
}

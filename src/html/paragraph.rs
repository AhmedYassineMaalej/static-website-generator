use crate::html::{HTMLNode, ToHtml};

pub struct Paragraph {
    pub children: Vec<HTMLNode>,
}

impl ToHtml for Paragraph {
    fn to_html(&self) -> String {
        let children = self.children.to_html();
        format!("<p>{children}</p>")
    }
}


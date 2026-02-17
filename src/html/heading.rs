use crate::html::{HTMLNode, ToHtml};

pub struct Heading {
    pub depth: u8,
    pub children: Vec<HTMLNode>,
}

impl ToHtml for Heading {
    fn to_html(&self) -> String {
        let tag = format!("h{}", self.depth);
        let children = self.children.to_html();
        format!("<{tag}>{children}</{tag}>")
    }
}

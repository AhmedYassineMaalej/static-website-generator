use crate::html::{HTMLNode, ToHtml};

pub struct Strong {
    pub children: Vec<HTMLNode>,
}

impl ToHtml for Strong {
    fn to_html(&self) -> String {
        format!("<b>{}</b>", self.children.to_html())
    }
}

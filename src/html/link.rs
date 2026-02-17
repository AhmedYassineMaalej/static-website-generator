use crate::html::{HTMLNode, ToHtml};

pub struct Link {
    pub children: Vec<HTMLNode>,
    pub url: String,
}

impl ToHtml for Link {
    fn to_html(&self) -> String {
        let url = self.url.clone();
        let children = self.children.to_html();
        format!("<a href=\"{url}\">{children}</a>",)
    }
}

use crate::{
    htmlnode::{HTMLNode, ToHTMLNode},
    leafnode::LeafNode,
    phrasing_content::PhrasingContent,
    resource::Resource,
};

#[derive(Debug)]
pub struct Link {
    children: Vec<PhrasingContent>,
    resource: Resource,
}

impl ToHTMLNode for Link {
    fn to_html_node(self) -> Box<dyn HTMLNode> {
        Box::new(
            LeafNode::new(
                "a",
                &self.resource.title.unwrap_or(self.resource.url.clone()),
            )
            .with_prop(String::from("href"), self.resource.url),
        )
    }
}

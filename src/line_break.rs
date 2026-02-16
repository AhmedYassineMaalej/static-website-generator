use crate::{
    htmlnode::{HTMLNode, ToHTMLNode},
    leafnode::LeafNode,
};

#[derive(Debug)]
pub struct LineBreak;

impl ToHTMLNode for LineBreak {
    fn to_html_node(self) -> Box<dyn HTMLNode> {
        Box::new(LeafNode::new("br", ""))
    }
}

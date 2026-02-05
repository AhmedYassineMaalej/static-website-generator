use crate::{leafnode::LeafNode, parentnode::ParentNode, properties::ToHtml};

#[derive(Debug, PartialEq)]
pub enum TextNode {
    Plain(String),
    Bold(Vec<TextNode>),
    Italic(Vec<TextNode>),
    Code(String),
    Link(Link),
    Image(Image),
}

#[derive(Debug, PartialEq)]
pub struct Link {
    text: String,
    url: String,
}

#[derive(Debug, PartialEq)]
pub struct Image {
    text: String,
    url: String,
}

impl TextNode {
    pub fn to_html_node(self) -> Box<dyn ToHtml> {
        match self {
            TextNode::Plain(text) => Box::new(text),
            TextNode::Bold(children) => Box::new(ParentNode::new(
                "b",
                children
                    .into_iter()
                    .map(|child| child.to_html_node())
                    .collect(),
            )),
            TextNode::Italic(children) => Box::new(ParentNode::new(
                "i",
                children
                    .into_iter()
                    .map(|child| child.to_html_node())
                    .collect(),
            )),
            TextNode::Code(code) => Box::new(LeafNode::new("code", &code)),
            TextNode::Link(Link { text, url }) => {
                Box::new(LeafNode::new("a", &text).with_prop(String::from("href"), url))
            }
            TextNode::Image(Image { text, url }) => {
                Box::new(LeafNode::new("img", &text).with_prop(String::from("src"), url))
            }
        }
    }
}

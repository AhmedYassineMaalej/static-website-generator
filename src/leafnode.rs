use crate::{
    htmlnode::HTMLNode,
    properties::{Properties, ToHtml},
};

#[derive(Debug)]
pub struct LeafNode {
    pub tag: String,
    pub value: String,
    pub props: Properties,
}

impl LeafNode {
    pub fn new(tag: &str, value: &str) -> Self {
        Self {
            tag: String::from(tag),
            value: String::from(value),
            props: Properties::new(),
        }
    }
}

impl HTMLNode for LeafNode {
    fn tag(&self) -> &String {
        &self.tag
    }

    fn props(&self) -> &Properties {
        &self.props
    }

    fn value(&self) -> Option<&String> {
        Some(&self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::properties::ToHtml;

    #[test]
    fn test_paragraph() {
        let node = LeafNode::new("p", "This is a paragraph of text.");

        assert_eq!(
            node.to_html(),
            String::from("<p>This is a paragraph of text.</p>")
        )
    }

    #[test]
    fn test_link() {
        let mut node = LeafNode::new("a", "Click me!");
        node.props
            .insert(String::from("href"), String::from("https://www.google.com"));

        assert_eq!(
            node.to_html(),
            String::from("<a href=\"https://www.google.com\">Click me!</a>")
        )
    }
}

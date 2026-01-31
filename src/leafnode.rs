use crate::{
    htmlnode::HTMLNode,
    properties::{Properties, ToHtml},
};

#[derive(Debug)]
pub struct LeafNode {
    tag: String,
    value: String,
    props: Properties,
}

impl LeafNode {
    pub fn new(tag: String, value: String, props: Properties) -> Self {
        Self { tag, value, props }
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
        let node = LeafNode::new(
            String::from("p"),
            String::from("This is a paragraph of text."),
            Properties::new(),
        );

        assert_eq!(
            node.to_html(),
            String::from("<p>This is a paragraph of text.</p>")
        )
    }

    #[test]
    fn test_link() {
        let mut props = Properties::new();
        props.insert(String::from("href"), String::from("https://www.google.com"));

        let node = LeafNode::new(String::from("a"), String::from("Click me!"), props);

        assert_eq!(
            node.to_html(),
            String::from("<a href=\"https://www.google.com\">Click me!</a>")
        )
    }
}

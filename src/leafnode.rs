use crate::{htmlnode::HTMLNode, properties::Properties};

#[derive(Debug, Default)]
pub struct LeafNode {
    tag: String,
    value: String,
    props: Properties,
}

impl LeafNode {
    fn new(tag: String, value: String, props: Properties) -> Self {
        Self { tag, value, props }
    }
}

impl HTMLNode for LeafNode {
    fn tag(&self) -> String {
        self.tag.clone()
    }

    fn props(&self) -> Properties {
        self.props.clone()
    }

    fn children(&self) -> Option<Vec<impl HTMLNode>> {
        Option::<Vec<LeafNode>>::None
    }

    fn value(&self) -> Option<String> {
        Some(self.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

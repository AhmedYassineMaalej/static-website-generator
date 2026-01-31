use crate::{
    htmlnode::HTMLNode,
    properties::{Properties, ToHtml},
};

#[derive(Default)]
pub struct ParentNode {
    pub tag: String,
    pub props: Properties,
    pub children: Vec<Box<dyn ToHtml>>,
}

impl ParentNode {
    pub fn new(tag: &str, children: Vec<Box<dyn ToHtml>>) -> Self {
        Self {
            tag: String::from(tag),
            props: Properties::new(),
            children,
        }
    }
}

impl HTMLNode for ParentNode {
    fn tag(&self) -> &String {
        &self.tag
    }

    fn props(&self) -> &Properties {
        &self.props
    }

    fn children(&self) -> Option<&Vec<Box<dyn ToHtml>>> {
        Some(&self.children)
    }
}

#[cfg(test)]
mod tests {
    use crate::leafnode::LeafNode;

    use super::*;

    #[test]
    fn test_many_children() {
        let children: Vec<Box<dyn ToHtml>> = vec![
            Box::new(LeafNode::new("b", "Bold text")),
            Box::new("Normal text"),
            Box::new(LeafNode::new("i", "italic text")),
            Box::new("Normal text"),
        ];

        let node = ParentNode::new("p", children);
        assert_eq!(
            node.to_html().as_str(),
            "<p><b>Bold text</b>Normal text<i>italic text</i>Normal text</p>"
        )
    }

    #[test]
    fn test_one_child() {
        let node = ParentNode::new("div", vec![Box::new(LeafNode::new("span", "child"))]);

        assert_eq!(node.to_html().as_str(), "<div><span>child</span></div>")
    }

    #[test]
    fn test_with_grandchild() {
        let node = ParentNode::new(
            "div",
            vec![Box::new(ParentNode::new(
                "span",
                vec![Box::new(LeafNode::new("b", "grandchild"))],
            ))],
        );

        assert_eq!(
            node.to_html().as_str(),
            "<div><span><b>grandchild</b></span></div>"
        )
    }
}

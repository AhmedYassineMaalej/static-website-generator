use crate::{
    htmlnode::HTMLNode,
    properties::{Properties, ToHtml},
};

#[derive(Default)]
pub struct ParentNode {
    tag: String,
    props: Properties,
    children: Vec<Box<dyn ToHtml>>,
}

impl ParentNode {
    pub fn new(tag: String, props: Properties, children: Vec<Box<dyn ToHtml>>) -> Self {
        Self {
            tag,
            props,
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
}

#[cfg(test)]
mod tests {
    use crate::leafnode::LeafNode;

    use super::*;

    #[test]
    fn test_parent_node() {
        let children: Vec<Box<dyn ToHtml>> = vec![
            Box::new(LeafNode::new(
                String::from("b"),
                String::from("Bold text"),
                Properties::new(),
            )),
            Box::new(LeafNode::new(
                String::from("b"),
                String::from("Bold text"),
                Properties::new(),
            )),
        ];

        let node = ParentNode::new(String::from("p"), Properties::new(), children);
    }
}

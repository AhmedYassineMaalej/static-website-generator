#[derive(Debug, PartialEq)]
pub enum TextNode {
    Plain { text: String },
    Bold { text: String },
    Italic { text: String },
    Code { text: String },
    Link { text: String, url: String },
    Image { text: String, url: String },
}

mod tests {
    use super::*;

    fn test_text_node_equal() {
        let node1 = TextNode::Bold {
            text: String::from("Hello World"),
        };
        let node2 = TextNode::Bold {
            text: String::from("Hello World"),
        };
        assert_eq!(node1, node2);
    }
}


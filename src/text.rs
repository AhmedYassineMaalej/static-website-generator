use std::collections::VecDeque;

use crate::{
    htmlnode::{HTMLNode, ToHTMLNode},
    leafnode::LeafNode,
    parser::Parsable,
    phrasing_content::PhrasingContent,
    token::Token,
};

impl Parsable for String {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        let str = tokens.pop_front()?.lexeme;
        Some(str)
    }
}

impl From<String> for PhrasingContent {
    fn from(val: String) -> Self {
        PhrasingContent::Text(val)
    }
}

impl ToHTMLNode for String {
    fn to_html_node(self) -> Box<dyn HTMLNode> {
        Box::new(self)
    }
}

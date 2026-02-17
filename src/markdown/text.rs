use std::collections::VecDeque;

use super::PhrasingContent;
use crate::html::{HTMLNode, ToHTMLNode};
use crate::parser::Parsable;
use crate::token::Token;

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
    fn to_html_node(self) -> HTMLNode {
        HTMLNode::Text(self)
    }
}

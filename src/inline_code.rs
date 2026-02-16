use std::collections::VecDeque;

use crate::{
    htmlnode::{HTMLNode, ToHTMLNode},
    leafnode::LeafNode,
    parser::Parsable,
    phrasing_content::PhrasingContent,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct InlineCode {
    code: String,
}

impl Parsable for InlineCode {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        let opening_token = &tokens[0];

        if opening_token.ttype != TokenType::Backtick {
            return None;
        }

        // find closing DoubleAsterisk
        let closing_idx = tokens
            .iter()
            .skip(1) // skip opening_token
            .position(|token| token.ttype == TokenType::Backtick)?
            + 1;

        // consume opening token
        tokens.pop_front();

        let code: String = tokens
            .drain(0..closing_idx - 1)
            .map(|token| token.lexeme)
            .collect();

        // consume closing token
        tokens.pop_front();

        Some(Self { code })
    }
}

impl From<InlineCode> for PhrasingContent {
    fn from(val: InlineCode) -> Self {
        PhrasingContent::InlineCode(val)
    }
}

impl ToHTMLNode for InlineCode {
    fn to_html_node(self) -> Box<dyn HTMLNode> {
        Box::new(LeafNode::new("p", &self.code))
    }
}

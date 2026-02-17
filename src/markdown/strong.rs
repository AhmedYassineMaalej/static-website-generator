use std::collections::VecDeque;

use crate::{
    html::{self, HTMLNode, ToHTMLNode},
    parser::Parsable,
    token::{Token, TokenType},
};

use super::PhrasingContent;

#[derive(Debug)]
pub struct Strong {
    children: Vec<PhrasingContent>,
}

impl Parsable for Strong {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        let opening_asterisk = &tokens[0];

        if opening_asterisk.ttype != TokenType::DoubleAsterisk {
            return None;
        }

        // find closing DoubleAsterisk
        let closing_idx = tokens
            .iter()
            .skip(1)
            .position(|token| token.ttype == TokenType::DoubleAsterisk)?
            + 1;

        // consume opening token
        tokens.pop_front();

        let mut children: VecDeque<Token> = tokens.drain(0..closing_idx - 1).collect();

        // consume closing token
        tokens.pop_front();

        Some(Self {
            children: Vec::<PhrasingContent>::parse(&mut children).unwrap(),
        })
    }
}

impl From<Strong> for PhrasingContent {
    fn from(val: Strong) -> Self {
        PhrasingContent::Strong(val)
    }
}

impl ToHTMLNode for Strong {
    fn to_html_node(self) -> HTMLNode {
        let children = self
            .children
            .into_iter()
            .map(|c| c.to_html_node())
            .collect();

        HTMLNode::Strong(html::Strong { children })
    }
}

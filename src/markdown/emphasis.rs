use std::collections::VecDeque;

use crate::html::{self, HTMLNode, ToHTMLNode};
use crate::parser::Parsable;
use crate::token::{Token, TokenType};

use super::PhrasingContent;

#[derive(Debug)]
pub struct Emphasis {
    children: Vec<PhrasingContent>,
}

impl Parsable for Emphasis {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        let opening_underscore = &tokens[0];

        if opening_underscore.ttype != TokenType::Underscore {
            return None;
        }

        // find closing DoubleAsterisk
        let closing_idx = tokens
            .iter()
            .skip(1)
            .position(|token| token.ttype == TokenType::Underscore)?
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

impl ToHTMLNode for Emphasis {
    fn to_html_node(self) -> HTMLNode {
        HTMLNode::Emphasis(html::Emphasis {
            children: self
                .children
                .into_iter()
                .map(|child| child.to_html_node())
                .collect(),
        })
    }
}

impl From<Emphasis> for PhrasingContent {
    fn from(val: Emphasis) -> Self {
        PhrasingContent::Emphasis(val)
    }
}

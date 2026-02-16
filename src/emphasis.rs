use std::collections::VecDeque;

use crate::{
    htmlnode::{HTMLNode, ToHTMLNode},
    parentnode::ParentNode,
    parser::Parsable,
    phrasing_content::PhrasingContent,
    token::{Token, TokenType},
};

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
    fn to_html_node(self) -> std::boxed::Box<(dyn HTMLNode + 'static)> {
        let children = self
            .children
            .into_iter()
            .map(|c| c.to_html_node())
            .collect();

        Box::new(ParentNode::new("i", children))
    }
}

impl From<Emphasis> for PhrasingContent {
    fn from(val: Emphasis) -> Self {
        PhrasingContent::Emphasis(val)
    }
}

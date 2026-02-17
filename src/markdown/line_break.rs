use std::collections::VecDeque;

use super::PhrasingContent;
use crate::html::{HTMLNode, ToHTMLNode};
use crate::parser::Parsable;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct LineBreak;

impl Parsable for LineBreak {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        tokens
            .pop_front_if(|token| token.ttype == TokenType::LineBreak)
            .map(|token| Self)
    }
}

impl From<LineBreak> for PhrasingContent {
    fn from(value: LineBreak) -> Self {
        Self::LineBreak(value)
    }
}

impl ToHTMLNode for LineBreak {
    fn to_html_node(self) -> HTMLNode {
        HTMLNode::Break
    }
}

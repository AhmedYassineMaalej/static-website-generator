use std::collections::VecDeque;

use crate::html::{HTMLNode, ToHTMLNode};
use crate::parser::Parsable;
use crate::token::Token;

use super::*;

#[derive(Debug)]
pub enum PhrasingContent {
    LineBreak(LineBreak),
    Emphasis(Emphasis),
    InlineCode(InlineCode),
    Link(Link),
    Strong(Strong),
    Text(String),
}

impl Parsable for PhrasingContent {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        if let Some(strong) = Strong::parse(tokens) {
            return Some(strong.into());
        }

        if let Some(emphasis) = Emphasis::parse(tokens) {
            return Some(emphasis.into());
        }

        if let Some(inline_code) = InlineCode::parse(tokens) {
            return Some(inline_code.into());
        }

        if let Some(link) = Link::parse(tokens) {
            return Some(link.into());
        }

        if let Some(line_break) = LineBreak::parse(tokens) {
            return Some(line_break.into());
        }

        Some(String::parse(tokens).unwrap().into())
    }
}

impl Parsable for Vec<PhrasingContent> {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        let mut res = Vec::new();

        while !tokens.is_empty() {
            res.push(PhrasingContent::parse(tokens)?);
        }

        Some(res)
    }
}

impl ToHTMLNode for PhrasingContent {
    fn to_html_node(self) -> HTMLNode {
        match self {
            PhrasingContent::LineBreak(line_break) => line_break.to_html_node(),
            PhrasingContent::Emphasis(emphasis) => emphasis.to_html_node(),
            PhrasingContent::InlineCode(inline_code) => inline_code.to_html_node(),
            PhrasingContent::Link(link) => link.to_html_node(),
            PhrasingContent::Strong(strong) => strong.to_html_node(),
            PhrasingContent::Text(text) => HTMLNode::Text(text),
        }
    }
}

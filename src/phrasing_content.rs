use std::collections::VecDeque;

use crate::{
    emphasis::Emphasis,
    htmlnode::{HTMLNode, ToHTMLNode},
    inline_code::InlineCode,
    leafnode::LeafNode,
    line_break::LineBreak,
    link::Link,
    parser::Parsable,
    strong::Strong,
    token::Token,
};

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
    fn to_html_node(self) -> Box<dyn HTMLNode> {
        match self {
            PhrasingContent::LineBreak(line_break) => line_break.to_html_node(),
            PhrasingContent::Emphasis(emphasis) => emphasis.to_html_node(),
            PhrasingContent::InlineCode(inline_code) => inline_code.to_html_node(),
            PhrasingContent::Link(link) => link.to_html_node(),
            PhrasingContent::Strong(strong) => strong.to_html_node(),
            PhrasingContent::Text(text) => Box::new(text),
        }
    }
}

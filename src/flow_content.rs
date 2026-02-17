use std::collections::VecDeque;

use crate::{heading::Heading, htmlnode::ToHTMLNode, parser::Parsable, token::Token};

#[derive(Debug)]
pub enum FlowContent {
    Heading(Heading),
}

impl Parsable for FlowContent {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        if let Some(heading) = Heading::parse(tokens) {
            return Some(heading.into());
        }

        None
    }
}

impl ToHTMLNode for FlowContent {
    fn to_html_node(self) -> Box<dyn crate::htmlnode::HTMLNode> {
        match self {
            FlowContent::Heading(heading) => heading.to_html_node(),
        }
    }
}

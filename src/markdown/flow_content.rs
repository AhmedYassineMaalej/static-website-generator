use std::collections::VecDeque;

use crate::html::{HTMLNode, ToHTMLNode};
use crate::parser::Parsable;
use crate::token::Token;

use super::{Code, Heading, Paragraph};

#[derive(Debug)]
pub enum FlowContent {
    Heading(Heading),
    Code(Code),
    Paragraph(Paragraph),
}

impl Parsable for FlowContent {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        if let Some(code) = Code::parse(tokens) {
            return Some(code.into());
        }

        if let Some(heading) = Heading::parse(tokens) {
            return Some(heading.into());
        }

        if let Some(paragraph) = Paragraph::parse(tokens) {
            return Some(paragraph.into());
        }

        None
    }
}

impl ToHTMLNode for FlowContent {
    fn to_html_node(self) -> HTMLNode {
        match self {
            FlowContent::Heading(heading) => heading.to_html_node(),
            FlowContent::Paragraph(paragraph) => paragraph.to_html_node(),
            FlowContent::Code(code) => code.to_html_node(),
        }
    }
}

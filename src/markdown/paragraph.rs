use std::collections::VecDeque;

use crate::parser::Parsable;
use crate::token::Token;

use super::{FlowContent, PhrasingContent};

#[derive(Debug)]
pub struct Paragraph {
    pub children: Vec<PhrasingContent>,
}

impl Parsable for Paragraph {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        todo!()
    }
}

impl From<Paragraph> for FlowContent {
    fn from(value: Paragraph) -> Self {
        FlowContent::Paragraph(value)
    }
}

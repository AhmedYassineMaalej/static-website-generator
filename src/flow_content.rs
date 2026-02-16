use std::collections::VecDeque;

use crate::{heading::Heading, parser::Parsable, token::Token};

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

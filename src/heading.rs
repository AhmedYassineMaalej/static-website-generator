use std::collections::VecDeque;

use crate::{
    flow_content::FlowContent, parser::Parsable, phrasing_content::PhrasingContent, token::Token,
};

#[derive(Debug)]
pub struct Heading {
    depth: u8,
    children: Vec<PhrasingContent>,
}

impl Parsable for Heading {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        todo!()
    }
}

impl From<Heading> for FlowContent {
    fn from(val: Heading) -> Self {
        FlowContent::Heading(val)
    }
}

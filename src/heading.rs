use std::collections::VecDeque;

use crate::{
    flow_content::FlowContent,
    htmlnode::{HTMLNode, ToHTMLNode},
    line_break::LineBreak,
    parentnode::ParentNode,
    parser::Parsable,
    phrasing_content::PhrasingContent,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Heading {
    depth: u8,
    children: Vec<PhrasingContent>,
}

impl Parsable for Heading {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        let mut depth = 0;
        let mut iter = tokens.iter().peekable();

        while let Some(token) = iter.next_if(|token| token.ttype == TokenType::Hashtag) {
            depth += 1;
        }

        if depth == 0 || depth > 6 {
            return None;
        }

        // make sure hashtags are followed by a space
        if !iter
            .next()
            .is_some_and(|token| token.ttype == TokenType::Space)
        {
            return None;
        }

        // find end of header
        let end_idx = iter.position(|token| token.ttype == TokenType::LineBreak)?;

        let mut children_tokens: VecDeque<Token> = tokens
            .drain(0..=depth + 1 + end_idx)
            .skip(depth + 1)
            .collect();

        // remove LineBreak
        children_tokens.pop_back();

        let children = Vec::<PhrasingContent>::parse(&mut children_tokens).unwrap();

        Some(Heading {
            depth: depth as u8,
            children,
        })
    }
}

impl From<Heading> for FlowContent {
    fn from(val: Heading) -> Self {
        FlowContent::Heading(val)
    }
}

impl ToHTMLNode for Heading {
    fn to_html_node(self) -> Box<dyn HTMLNode> {
        let children = self
            .children
            .into_iter()
            .map(|child| child.to_html_node())
            .collect();

        let tag = format!("h{}", self.depth);
        Box::new(ParentNode::new(&tag, children))
    }
}

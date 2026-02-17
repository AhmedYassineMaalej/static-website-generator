use std::{boxed::Box, collections::VecDeque};

use super::FlowContent;
use super::PhrasingContent;
use crate::html;
use crate::html::{HTMLNode, ToHTMLNode};
use crate::parser::Parsable;
use crate::token::Token;

#[derive(Debug)]
pub struct Root {
    pub children: Vec<MdAstContent>,
}

#[derive(Debug)]
pub enum MdAstContent {
    PhrasingContent(PhrasingContent),
    FlowContent(FlowContent),
}

impl ToHTMLNode for MdAstContent {
    fn to_html_node(self) -> HTMLNode {
        match self {
            MdAstContent::PhrasingContent(content) => content.to_html_node(),
            MdAstContent::FlowContent(content) => content.to_html_node(),
        }
    }
}

impl Parsable for Root {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        let mut children = Vec::new();

        while !tokens.is_empty() {
            if let Some(content) = FlowContent::parse(tokens) {
                children.push(MdAstContent::FlowContent(content));
            } else {
                let content = PhrasingContent::parse(tokens)?;
                children.push(MdAstContent::PhrasingContent(content));
            }
        }

        Some(Self { children })
    }
}

impl ToHTMLNode for Root {
    fn to_html_node(self) -> HTMLNode {
        let children = self
            .children
            .into_iter()
            .map(|c| c.to_html_node())
            .collect();

        HTMLNode::Body(html::Body { children })
    }
}

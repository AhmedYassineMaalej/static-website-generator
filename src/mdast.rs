use std::{boxed::Box, collections::VecDeque};

use crate::{
    flow_content::FlowContent,
    htmlnode::{HTMLNode, ToHTMLNode},
    parentnode::ParentNode,
    parser::Parsable,
    phrasing_content::PhrasingContent,
    properties::ToHtml,
    token::Token,
};

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
    fn to_html_node(self) -> Box<dyn HTMLNode> {
        match self {
            MdAstContent::PhrasingContent(content) => content.to_html_node(),
            MdAstContent::FlowContent(content) => todo!(),
        }
    }
}

impl Parsable for Root {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        let mut children = Vec::new();

        while !tokens.is_empty() {
            // if let Some(content) = FlowContent::parse(tokens) {
            //     children.push(MdAstContent::FlowContent(content));
            // } else {
            let content = PhrasingContent::parse(tokens)?;
            children.push(MdAstContent::PhrasingContent(content));
            // }
        }

        Some(Self { children })
    }
}

impl ToHTMLNode for Root {
    fn to_html_node(self) -> Box<dyn HTMLNode> {
        let children = self
            .children
            .into_iter()
            .map(|c| c.to_html_node())
            .collect();
        Box::new(ParentNode::new("body", children))
    }
}

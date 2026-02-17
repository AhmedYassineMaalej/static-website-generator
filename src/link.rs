use std::collections::VecDeque;

use crate::{
    htmlnode::{HTMLNode, ToHTMLNode},
    leafnode::LeafNode,
    parentnode::ParentNode,
    parser::Parsable,
    phrasing_content::PhrasingContent,
    resource::Resource,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Link {
    children: Vec<PhrasingContent>,
    resource: Resource,
}

impl Parsable for Link {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        let mut iter = tokens.iter().peekable();
        if !iter
            .next()
            .is_some_and(|token| token.ttype == TokenType::OpenBracket)
        {
            return None;
        }

        let mut children_size = 0;
        while let Some(token) = iter.next_if(|token| token.ttype != TokenType::CloseBracket) {
            children_size += 1;
        }

        if !iter
            .next()
            .is_some_and(|token| token.ttype == TokenType::CloseBracket)
        {
            return None;
        }

        if !iter
            .next()
            .is_some_and(|token| token.ttype == TokenType::OpenParenthesis)
        {
            return None;
        }

        let mut resource_size = 0;
        while let Some(token) = iter.next_if(|token| token.ttype != TokenType::CloseParenthesis) {
            resource_size += 1;
        }

        if !iter
            .next()
            .is_some_and(|token| token.ttype == TokenType::CloseParenthesis)
        {
            return None;
        }

        tokens.pop_front(); // consume opening bracket
        let mut children_tokens = tokens.drain(0..children_size).collect();
        tokens.pop_front(); // consume closing bracket
        tokens.pop_front(); // consume open parenthesis
        let mut resources_tokens = tokens.drain(0..resource_size).collect();
        tokens.pop_front();

        Some(Link {
            children: Vec::<PhrasingContent>::parse(&mut children_tokens).unwrap(),
            resource: Resource::parse(&mut resources_tokens)?,
        })
    }
}

impl From<Link> for PhrasingContent {
    fn from(value: Link) -> Self {
        Self::Link(value)
    }
}

impl ToHTMLNode for Link {
    fn to_html_node(self) -> Box<dyn HTMLNode> {
        let children = self
            .children
            .into_iter()
            .map(|child| child.to_html_node())
            .collect();

        let mut node = ParentNode::new("a", children);
        node.props.insert(String::from("href"), self.resource.url);

        Box::new(node)
    }
}

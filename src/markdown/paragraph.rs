use std::collections::VecDeque;

use crate::html::{self, HTMLNode, ToHTMLNode};
use crate::token::Token;
use crate::{parser::Parsable, token::TokenType};

use super::{FlowContent, PhrasingContent};

#[derive(Debug)]
pub struct Paragraph {
    pub children: Vec<PhrasingContent>,
}

impl Parsable for Paragraph {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        let mut children_tokens: VecDeque<Token> = VecDeque::new();

        let mut prev_token_line_break = false;
        while let Some(token) = tokens.pop_front() {
            if token.ttype != TokenType::LineBreak {
                prev_token_line_break = false;
                children_tokens.push_back(token);
                continue;
            }

            if prev_token_line_break {
                break;
            }

            prev_token_line_break = true;

            children_tokens.push_back(Token {
                lexeme: String::from(" "),
                ttype: TokenType::Space,
            });
        }

        Some(Self {
            children: Vec::<PhrasingContent>::parse(&mut children_tokens).unwrap(),
        })
    }
}

impl From<Paragraph> for FlowContent {
    fn from(value: Paragraph) -> Self {
        FlowContent::Paragraph(value)
    }
}

impl ToHTMLNode for Paragraph {
    fn to_html_node(self) -> HTMLNode {
        let children = self
            .children
            .into_iter()
            .map(|child| child.to_html_node())
            .collect();

        HTMLNode::Paragraph(html::Paragraph { children })
    }
}

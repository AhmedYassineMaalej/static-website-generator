use std::collections::VecDeque;

use crate::{
    html::{self, HTMLNode, ToHTMLNode},
    markdown::FlowContent,
    parser::Parsable,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Code {
    code: String,
    language: Option<String>,
}

impl Parsable for Code {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        if !tokens
            .front()
            .is_some_and(|token| token.ttype == TokenType::TripleBacktick)
        {
            return None;
        }

        let mut iter = tokens.iter();
        let mut prev = iter.next().unwrap();
        let mut len = 0;

        loop {
            let token = iter.next()?;
            if token.ttype == TokenType::TripleBacktick && prev.ttype == TokenType::LineBreak {
                break;
            }
            prev = token;
            len += 1;
        }

        tokens.pop_front(); // remove opening ticks
        let code = tokens
            .drain(0..len)
            .skip(1) // skip line break
            .map(|token| token.lexeme)
            .collect();
        tokens.pop_front(); // remove closing ticks
        tokens.pop_front(); // remove linebreak
        Some(Code {
            code,
            language: None,
        })
    }
}

impl ToHTMLNode for Code {
    fn to_html_node(self) -> HTMLNode {
        HTMLNode::Code(html::Code {
            code: self.code,
            language: self.language,
        })
    }
}

impl From<Code> for FlowContent {
    fn from(value: Code) -> Self {
        Self::Code(value)
    }
}

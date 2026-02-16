use std::collections::VecDeque;

use crate::token::{Token, TokenType};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: usize,
}

pub trait Parsable: Sized {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self>;
}

impl Parser {
    // pub fn parse(tokens: Vec<Token>) -> Root {
    //     let mut parser = Self::new(tokens);
    //     parser.process_tokens();
    // }

    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::Tokenizer;

    use super::*;
    //
    // #[test]
    // fn test_plain() {
    //     let input = String::from("Hello World");
    //     let tokens = Tokenizer::tokenize(input);
    //
    //     assert_eq!(
    //         Parser::parse(tokens),
    //         vec![TextNode::Plain(String::from("Hello World"))]
    //     )
    // }
    //
    // #[test]
    // fn test_bold() {
    //     let input = String::from("**Hello World**");
    //     let tokens = Tokenizer::tokenize(input);
    //
    //     assert_eq!(
    //         Parser::parse(tokens),
    //         vec![TextNode::Bold(vec![TextNode::Plain(String::from(
    //             "Hello World"
    //         ))])]
    //     )
    // }
    //
    // #[test]
    // fn test_italic() {
    //     let input = String::from("_Hello World_");
    //     let tokens = Tokenizer::tokenize(input);
    //
    //     assert_eq!(
    //         Parser::parse(tokens),
    //         vec![TextNode::Italic(vec![TextNode::Plain(String::from(
    //             "Hello World"
    //         ))])]
    //     )
    // }
    //
    // #[test]
    // fn test_code() {
    //     let input = String::from("`Hello World`");
    //     let tokens = Tokenizer::tokenize(input);
    //
    //     assert_eq!(
    //         Parser::parse(tokens),
    //         vec![TextNode::Code(String::from("Hello World"))]
    //     )
    // }
}

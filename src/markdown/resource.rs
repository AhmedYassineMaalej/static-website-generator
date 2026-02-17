use std::collections::VecDeque;

use crate::{
    parser::Parsable,
    token::{Token, TokenType},
};

#[derive(Debug, PartialEq)]
pub struct Resource {
    pub url: String,
    pub title: Option<String>,
}

impl Parsable for Resource {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
        let url = tokens.pop_front().unwrap().lexeme;

        if tokens.is_empty() {
            return Some(Self { url, title: None });
        }

        tokens.pop_front_if(|token| token.ttype == TokenType::Space)?;

        let title = tokens
            .pop_front_if(|token| token.ttype == TokenType::Space)
            .map(|token| token.lexeme);

        Some(Self { url, title })
    }
}

mod tests {
    use super::*;

    fn test_parse() {
        let mut tokens = VecDeque::from(vec![
            Token {
                lexeme: String::from("https://github.com"),
                ttype: TokenType::Text,
            },
            Token {
                lexeme: String::from(" "),
                ttype: TokenType::Space,
            },
            Token {
                lexeme: String::from("link to repo"),
                ttype: TokenType::Text,
            },
        ]);

        assert_eq!(
            Resource::parse(&mut tokens),
            Some(Resource {
                url: String::from("https://github.com"),
                title: Some(String::from("link to repo"))
            })
        );
    }

    fn test_parse_no_title() {
        let mut tokens = VecDeque::from(vec![Token {
            lexeme: String::from("https://github.com"),
            ttype: TokenType::Text,
        }]);

        assert_eq!(
            Resource::parse(&mut tokens),
            Some(Resource {
                url: String::from("https://github.com"),
                title: None,
            })
        );
    }
}

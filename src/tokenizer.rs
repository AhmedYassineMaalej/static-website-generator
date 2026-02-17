use crate::token::{Token, TokenType};

pub struct Tokenizer {
    input: String,
    current_token: String,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn tokenize(input: String) -> Vec<Token> {
        let mut tokenizer = Self::new(input);
        tokenizer.process_input();
        tokenizer.tokens
    }

    fn new(input: String) -> Self {
        Self {
            input,
            current_token: String::new(),
            tokens: Vec::new(),
        }
    }

    fn process_input(&mut self) {
        let input = self.input.clone();
        let mut chars = input.chars().peekable();

        while let Some(char) = chars.next() {
            if char == '*' && chars.peek() == Some(&'*') {
                self.finalize_text();
                chars.next();
                self.tokens.push(Token {
                    lexeme: String::from("**"),
                    ttype: TokenType::DoubleAsterisk,
                });
                continue;
            }
            if char == '(' {
                self.finalize_text();
                self.tokens.push(Token {
                    lexeme: String::from("("),
                    ttype: TokenType::OpenParenthesis,
                });
                continue;
            }

            if char == ')' {
                self.finalize_text();
                self.tokens.push(Token {
                    lexeme: String::from(")"),
                    ttype: TokenType::CloseParenthesis,
                });
                continue;
            }

            if char == ']' {
                self.finalize_text();
                self.tokens.push(Token {
                    lexeme: String::from("]"),
                    ttype: TokenType::CloseBracket,
                });
                continue;
            }

            if char == '[' {
                self.finalize_text();
                self.tokens.push(Token {
                    lexeme: String::from("["),
                    ttype: TokenType::OpenBracket,
                });
                continue;
            }

            if char == '\n' {
                self.finalize_text();
                self.tokens.push(Token {
                    lexeme: String::from("\n"),
                    ttype: TokenType::LineBreak,
                });
                continue;
            }

            if char == '#' {
                self.finalize_text();
                self.tokens.push(Token {
                    lexeme: String::from("#"),
                    ttype: TokenType::Hashtag,
                });
                continue;
            }

            if char == ' ' {
                self.finalize_text();
                self.tokens.push(Token {
                    lexeme: String::from(" "),
                    ttype: TokenType::Space,
                });
                continue;
            }

            if char == '_' {
                self.finalize_text();
                self.tokens.push(Token {
                    lexeme: String::from("_"),
                    ttype: TokenType::Underscore,
                });
                continue;
            }

            if char == '`' {
                self.finalize_text();
                self.tokens.push(Token {
                    lexeme: String::from("`"),
                    ttype: TokenType::Backtick,
                });
                continue;
            }

            if char == '#' {
                self.finalize_text();
                self.tokens.push(Token {
                    lexeme: String::from("`"),
                    ttype: TokenType::Backtick,
                });
                continue;
            }

            self.current_token.push(char);
        }

        self.finalize_text();
    }

    pub fn finalize_text(&mut self) {
        if self.current_token.is_empty() {
            return;
        }

        self.tokens.push(Token {
            lexeme: std::mem::take(&mut self.current_token),
            ttype: TokenType::Text,
        });
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let input = String::from("Hello world");

        assert_eq!(
            Tokenizer::tokenize(input),
            vec![
                Token::try_from("Hello").unwrap(),
                Token::try_from(" ").unwrap(),
                Token::try_from("world").unwrap(),
            ]
        )
    }

    #[test]
    fn test_surrounding_asterisks() {
        let input = String::from("**Hello world**");

        assert_eq!(
            Tokenizer::tokenize(input),
            vec![
                Token::try_from("**").unwrap(),
                Token::try_from("Hello").unwrap(),
                Token::try_from(" ").unwrap(),
                Token::try_from("world").unwrap(),
                Token::try_from("**").unwrap(),
            ]
        )
    }

    #[test]
    fn test_middle_asterisks() {
        let input = String::from("**Hello**world");

        assert_eq!(
            Tokenizer::tokenize(input),
            vec![
                Token::try_from("**").unwrap(),
                Token::try_from("Hello").unwrap(),
                Token::try_from("**").unwrap(),
                Token::try_from("world").unwrap(),
            ]
        )
    }

    #[test]
    fn test_surrounding_underscores() {
        let input = String::from("_Hello world_");

        assert_eq!(
            Tokenizer::tokenize(input),
            vec![
                Token::try_from("_").unwrap(),
                Token::try_from("Hello").unwrap(),
                Token::try_from(" ").unwrap(),
                Token::try_from("world").unwrap(),
                Token::try_from("_").unwrap(),
            ]
        )
    }

    #[test]
    fn test_middle_underscores() {
        let input = String::from("_Hello_world");

        assert_eq!(
            Tokenizer::tokenize(input),
            vec![
                Token::try_from("_").unwrap(),
                Token::try_from("Hello").unwrap(),
                Token::try_from("_").unwrap(),
                Token::try_from("world").unwrap(),
            ]
        )
    }

    #[test]
    fn test_surrounding_backticks() {
        let input = String::from("`Hello world`");

        assert_eq!(
            Tokenizer::tokenize(input),
            vec![
                Token::try_from("`").unwrap(),
                Token::try_from("Hello").unwrap(),
                Token::try_from(" ").unwrap(),
                Token::try_from("world").unwrap(),
                Token::try_from("`").unwrap(),
            ]
        )
    }

    #[test]
    fn test_middle_backticks() {
        let input = String::from("`Hello`world");

        assert_eq!(
            Tokenizer::tokenize(input),
            vec![
                Token::try_from("`").unwrap(),
                Token::try_from("Hello").unwrap(),
                Token::try_from("`").unwrap(),
                Token::try_from("world").unwrap(),
            ]
        )
    }

    #[test]
    fn test_three_in_one() {
        let input = String::from("`_Hello**_");

        assert_eq!(
            Tokenizer::tokenize(input),
            vec![
                Token::try_from("`").unwrap(),
                Token::try_from("_").unwrap(),
                Token::try_from("Hello").unwrap(),
                Token::try_from("**").unwrap(),
                Token::try_from("_").unwrap(),
            ]
        )
    }
}

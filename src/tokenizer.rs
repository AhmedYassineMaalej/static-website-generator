use crate::token::{Token, TokenType};

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
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
            input: input.chars().collect(),
            current_token: String::new(),
            position: 0,
            tokens: Vec::new(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).cloned()
    }

    fn peek_nth(&self, n: usize) -> Option<char> {
        self.input.get(self.position + n).cloned()
    }

    fn next(&mut self) -> Option<char> {
        let res = self.input.get(self.position);
        self.position += 1;
        res.cloned()
    }

    fn push_token(&mut self, token: Token) {
        self.finalize_text();
        self.tokens.push(token);
    }

    fn process_input(&mut self) {
        while let Some(char) = self.next() {
            let token = match char {
                '*' if self.peek() == Some('*') => {
                    self.next(); // consume 2nd asterisk
                    Token {
                        lexeme: String::from("**"),
                        ttype: TokenType::DoubleAsterisk,
                    }
                }
                '(' => Token {
                    lexeme: String::from("("),
                    ttype: TokenType::OpenParenthesis,
                },
                ')' => Token {
                    lexeme: String::from(")"),
                    ttype: TokenType::CloseParenthesis,
                },
                ']' => Token {
                    lexeme: String::from("]"),
                    ttype: TokenType::CloseBracket,
                },
                '[' => Token {
                    lexeme: String::from("["),
                    ttype: TokenType::OpenBracket,
                },
                '\n' => Token {
                    lexeme: String::from("\n"),
                    ttype: TokenType::LineBreak,
                },

                '#' => Token {
                    lexeme: String::from("#"),
                    ttype: TokenType::Hashtag,
                },
                ' ' => Token {
                    lexeme: String::from(" "),
                    ttype: TokenType::Space,
                },
                '_' => Token {
                    lexeme: String::from("_"),
                    ttype: TokenType::Underscore,
                },
                '`' if self.peek() == Some('`') && self.peek_nth(1) == Some('`') => {
                    self.next();
                    self.next();
                    Token {
                        lexeme: String::from("```"),
                        ttype: TokenType::TripleBacktick,
                    }
                }
                '`' => Token {
                    lexeme: String::from("`"),
                    ttype: TokenType::Backtick,
                },
                '#' => Token {
                    lexeme: String::from("`"),
                    ttype: TokenType::Backtick,
                },
                c => {
                    self.current_token.push(c);
                    continue;
                }
            };

            self.push_token(token);
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

    #[test]
    fn test_triple_backticks() {
        let input = String::from("```");

        assert_eq!(
            Tokenizer::tokenize(input),
            vec![Token::try_from("```").unwrap()]
        )
    }
}

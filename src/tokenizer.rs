#[derive(Debug, PartialEq)]
pub struct Token {
    pub lexeme: String,
    pub ttype: TokenType,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Text,
    DoubleAsterisk,
    Underscore,
    Backtick,
}

pub struct Tokenizer {
    pub input: String,
    pub current_token: String,
    pub tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            current_token: String::new(),
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) {
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

        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokenize();
        assert_eq!(
            tokenizer.tokens,
            vec![Token {
                lexeme: String::from("Hello world"),
                ttype: TokenType::Text,
            }]
        )
    }

    #[test]
    fn test_surrounding_asterisks() {
        let input = String::from("**Hello world**");

        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokenize();
        assert_eq!(
            tokenizer.tokens,
            vec![
                Token {
                    lexeme: String::from("**"),
                    ttype: TokenType::DoubleAsterisk,
                },
                Token {
                    lexeme: String::from("Hello world"),
                    ttype: TokenType::Text,
                },
                Token {
                    lexeme: String::from("**"),
                    ttype: TokenType::DoubleAsterisk,
                },
            ]
        )
    }

    #[test]
    fn test_middle_asterisks() {
        let input = String::from("**Hello**world");

        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokenize();
        assert_eq!(
            tokenizer.tokens,
            vec![
                Token {
                    lexeme: String::from("**"),
                    ttype: TokenType::DoubleAsterisk,
                },
                Token {
                    lexeme: String::from("Hello"),
                    ttype: TokenType::Text,
                },
                Token {
                    lexeme: String::from("**"),
                    ttype: TokenType::DoubleAsterisk,
                },
                Token {
                    lexeme: String::from("world"),
                    ttype: TokenType::Text,
                },
            ]
        )
    }

    #[test]
    fn test_surrounding_underscores() {
        let input = String::from("_Hello world_");

        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokenize();
        assert_eq!(
            tokenizer.tokens,
            vec![
                Token {
                    lexeme: String::from("_"),
                    ttype: TokenType::Underscore,
                },
                Token {
                    lexeme: String::from("Hello world"),
                    ttype: TokenType::Text,
                },
                Token {
                    lexeme: String::from("_"),
                    ttype: TokenType::Underscore,
                },
            ]
        )
    }

    #[test]
    fn test_middle_underscores() {
        let input = String::from("_Hello_world");

        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokenize();
        assert_eq!(
            tokenizer.tokens,
            vec![
                Token {
                    lexeme: String::from("_"),
                    ttype: TokenType::Underscore,
                },
                Token {
                    lexeme: String::from("Hello"),
                    ttype: TokenType::Text,
                },
                Token {
                    lexeme: String::from("_"),
                    ttype: TokenType::Underscore,
                },
                Token {
                    lexeme: String::from("world"),
                    ttype: TokenType::Text,
                },
            ]
        )
    }

    #[test]
    fn test_surrounding_backticks() {
        let input = String::from("`Hello world`");

        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokenize();
        assert_eq!(
            tokenizer.tokens,
            vec![
                Token {
                    lexeme: String::from("`"),
                    ttype: TokenType::Backtick,
                },
                Token {
                    lexeme: String::from("Hello world"),
                    ttype: TokenType::Text,
                },
                Token {
                    lexeme: String::from("`"),
                    ttype: TokenType::Backtick,
                },
            ]
        )
    }

    #[test]
    fn test_middle_backticks() {
        let input = String::from("`Hello`world");

        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokenize();
        assert_eq!(
            tokenizer.tokens,
            vec![
                Token {
                    lexeme: String::from("`"),
                    ttype: TokenType::Backtick,
                },
                Token {
                    lexeme: String::from("Hello"),
                    ttype: TokenType::Text,
                },
                Token {
                    lexeme: String::from("`"),
                    ttype: TokenType::Backtick,
                },
                Token {
                    lexeme: String::from("world"),
                    ttype: TokenType::Text,
                },
            ]
        )
    }

    #[test]
    fn test_three_in_one() {
        let input = String::from("`_Hello**_");

        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokenize();
        assert_eq!(
            tokenizer.tokens,
            vec![
                Token {
                    lexeme: String::from("`"),
                    ttype: TokenType::Backtick,
                },
                Token {
                    lexeme: String::from("_"),
                    ttype: TokenType::Underscore,
                },
                Token {
                    lexeme: String::from("Hello"),
                    ttype: TokenType::Text,
                },
                Token {
                    lexeme: String::from("**"),
                    ttype: TokenType::DoubleAsterisk,
                },
                Token {
                    lexeme: String::from("_"),
                    ttype: TokenType::Underscore,
                },
            ]
        )
    }
}

pub const SPECIAL_CHARS: [char; 9] = ['*', '_', '`', '#', ' ', '[', ']', '(', ')'];

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Text,
    DoubleAsterisk,
    OpenBracket,
    CloseBracket,
    OpenParenthesis,
    CloseParenthesis,
    Underscore,
    Backtick,
    Hashtag,
    Space,
    LineBreak,
}

impl From<&str> for TokenType {
    fn from(value: &str) -> Self {
        match value {
            "**" => Self::DoubleAsterisk,
            "_" => Self::Underscore,
            "`" => Self::Backtick,
            "#" => Self::Hashtag,
            " " => Self::Space,
            _ => Self::Text,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub lexeme: String,
    pub ttype: TokenType,
}

#[derive(Debug, PartialEq)]
pub enum TokenError {
    InvalidToken,
}

impl TryFrom<&str> for Token {
    type Error = TokenError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let ttype = TokenType::from(value);

        if ttype == TokenType::Text && value.contains(SPECIAL_CHARS) {
            return Err(TokenError::InvalidToken);
        }

        Ok(Self {
            lexeme: String::from(value),
            ttype: TokenType::from(value),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_token_try_from() {
        assert_eq!(
            Token::try_from("hello"),
            Ok(Token {
                lexeme: String::from("hello"),
                ttype: TokenType::Text
            })
        );
        assert_eq!(
            Token::try_from("**"),
            Ok(Token {
                lexeme: String::from("**"),
                ttype: TokenType::DoubleAsterisk
            })
        );
        assert_eq!(
            Token::try_from("_"),
            Ok(Token {
                lexeme: String::from("_"),
                ttype: TokenType::Underscore
            })
        );
        assert_eq!(
            Token::try_from("`"),
            Ok(Token {
                lexeme: String::from("`"),
                ttype: TokenType::Backtick
            })
        );
    }

    #[test]
    fn test_invalid_token_try_from() {
        assert_eq!(Token::try_from("*"), Err(TokenError::InvalidToken));
        assert_eq!(Token::try_from("Hello**"), Err(TokenError::InvalidToken));
        assert_eq!(Token::try_from("`*"), Err(TokenError::InvalidToken));
        assert_eq!(Token::try_from("__"), Err(TokenError::InvalidToken));
        assert_eq!(
            Token::try_from("Hello_world"),
            Err(TokenError::InvalidToken)
        );
    }
}

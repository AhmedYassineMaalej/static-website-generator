use crate::{
    textnode::TextNode,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    nodes: Vec<TextNode>,
    position: usize,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Vec<TextNode> {
        let mut parser = Self::new(tokens);
        parser.process_tokens();
        parser.nodes
    }

    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            nodes: Vec::new(),
            position: 0,
        }
    }

    fn process_tokens(&mut self) {
        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            let parse_res = match token.ttype {
                TokenType::Text => {
                    self.position += 1;
                    Ok(TextNode::Plain(token.lexeme.clone()))
                }
                TokenType::DoubleAsterisk => self.parse_bold(),
                TokenType::Underscore => self.parse_italic(),
                TokenType::Backtick => self.parse_code(),
            };

            match parse_res {
                Ok(node) => self.nodes.push(node),
                Err(nodes) => self.nodes.extend(nodes),
            }
        }
    }

    fn parse_bold(&mut self) -> Result<TextNode, Vec<TextNode>> {
        assert_eq!(
            &self.tokens[self.position].ttype,
            &TokenType::DoubleAsterisk
        );
        self.position += 1;

        let mut children = Vec::new();

        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            match token.ttype {
                TokenType::DoubleAsterisk => {
                    self.position += 1;
                    return Ok(TextNode::Bold(children));
                }
                TokenType::Text => {
                    children.push(TextNode::Plain(token.lexeme.clone()));
                    self.position += 1;
                }
                TokenType::Underscore => match self.parse_italic() {
                    Ok(italic_node) => children.push(italic_node),
                    Err(nodes) => children.extend(nodes),
                },
                TokenType::Backtick => match self.parse_code() {
                    Ok(code_node) => children.push(code_node),
                    Err(nodes) => children.extend(nodes),
                },
            }
        }

        children.insert(0, TextNode::Plain(String::from("**")));
        Err(children)
    }

    fn parse_italic(&mut self) -> Result<TextNode, Vec<TextNode>> {
        assert_eq!(&self.tokens[self.position].ttype, &TokenType::Underscore);
        self.position += 1;

        let mut children = Vec::new();

        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            match token.ttype {
                TokenType::Underscore => {
                    self.position += 1;
                    return Ok(TextNode::Italic(children));
                }
                TokenType::Text => {
                    children.push(TextNode::Plain(token.lexeme.clone()));
                    self.position += 1;
                }
                TokenType::DoubleAsterisk => match self.parse_bold() {
                    Ok(bold_node) => children.push(bold_node),
                    Err(nodes) => children.extend(nodes),
                },
                TokenType::Backtick => match self.parse_code() {
                    Ok(code_node) => children.push(code_node),
                    Err(nodes) => children.extend(nodes),
                },
            }
        }

        children.insert(0, TextNode::Plain(String::from("_")));
        Err(children)
    }

    fn parse_code(&mut self) -> Result<TextNode, Vec<TextNode>> {
        assert_eq!(&self.tokens[self.position].ttype, &TokenType::Backtick);

        // find closing backtick
        let mut idx = None;
        for i in self.position + 1..self.tokens.len() {
            if self.tokens[i].ttype == TokenType::Backtick {
                idx = Some(i);
                break;
            }
        }

        // no closing backtick
        if idx.is_none() {
            let res = Ok(TextNode::Plain(self.tokens[self.position].lexeme.clone()));
            self.position += 1;
            return res;
        }

        // found closing backtick
        let idx = idx.unwrap();
        self.position += 1; // consume opening backtick
        let mut code = String::new();
        for i in self.position..idx {
            self.position += 1;
            code.push_str(&self.tokens[i].lexeme);
        }

        self.position += 1; // consume closing backtick

        Ok(TextNode::Code(code))
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::Tokenizer;

    use super::*;

    #[test]
    fn test_plain() {
        let input = String::from("Hello World");
        let tokens = Tokenizer::tokenize(input);

        assert_eq!(
            Parser::parse(tokens),
            vec![TextNode::Plain(String::from("Hello World"))]
        )
    }

    #[test]
    fn test_bold() {
        let input = String::from("**Hello World**");
        let tokens = Tokenizer::tokenize(input);

        assert_eq!(
            Parser::parse(tokens),
            vec![TextNode::Bold(vec![TextNode::Plain(String::from(
                "Hello World"
            ))])]
        )
    }

    #[test]
    fn test_italic() {
        let input = String::from("_Hello World_");
        let tokens = Tokenizer::tokenize(input);

        assert_eq!(
            Parser::parse(tokens),
            vec![TextNode::Italic(vec![TextNode::Plain(String::from(
                "Hello World"
            ))])]
        )
    }

    #[test]
    fn test_code() {
        let input = String::from("`Hello World`");
        let tokens = Tokenizer::tokenize(input);

        assert_eq!(
            Parser::parse(tokens),
            vec![TextNode::Code(String::from("Hello World"))]
        )
    }
}

use crate::{
    textnode::TextNode,
    tokenizer::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    nodes: Vec<TextNode>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            nodes: Vec::new(),
            position: 0,
        }
    }

    pub fn parse(&mut self) {
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

    pub fn parse_bold(&mut self) -> Result<TextNode, Vec<TextNode>> {
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
                TokenType::Text => children.push(TextNode::Plain(token.lexeme.clone())),
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

    pub fn parse_italic(&mut self) -> Result<TextNode, Vec<TextNode>> {
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
                TokenType::Text => children.push(TextNode::Plain(token.lexeme.clone())),
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

    pub fn parse_code(&mut self) -> Result<TextNode, Vec<TextNode>> {
        assert_eq!(&self.tokens[self.position].ttype, &TokenType::Backtick);

        // find closing backtick
        let mut idx = None;
        for i in self.position + 1..self.tokens.len() {
            if self.tokens[i].ttype == TokenType::Backtick {
                idx = Some(i);
                break;
            }
        }

        if idx.is_none() {
            let res = Ok(TextNode::Plain(self.tokens[self.position].lexeme.clone()));
            self.position += 1;
            return res;
        }

        let idx = idx.unwrap();
        let mut code = String::new();
        for i in self.position + 1..idx {
            code.push_str(&self.tokens[i].lexeme);
        }

        Ok(TextNode::Code(code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain() {}
}

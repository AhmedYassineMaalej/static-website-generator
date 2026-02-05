mod htmlnode;
mod leafnode;
mod parentnode;
mod parser;
mod properties;
mod textnode;
mod token;
mod tokenizer;

use textnode::TextNode;

use crate::{parser::Parser, tokenizer::Tokenizer};

fn main() {
    let input = String::from("`hello markdown`");
    let tokens = Tokenizer::tokenize(input);
    let nodes = Parser::parse(tokens);
    let html = nodes.into_iter().next().unwrap().to_html_node().to_html();

    std::fs::write("public/index.html", html);
}

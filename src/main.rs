#![allow(unused)]

mod emphasis;
mod flow_content;
mod heading;
mod htmlnode;
mod inline_code;
mod leafnode;
mod line_break;
mod link;
mod mdast;
mod parentnode;
mod parser;
mod phrasing_content;
mod properties;
mod resource;
mod strong;
mod text;
mod textnode;
mod token;
mod tokenizer;

use std::{collections::VecDeque, fs};

use crate::{
    htmlnode::ToHTMLNode, mdast::Root, parser::Parsable, properties::ToHtml, tokenizer::Tokenizer,
};

fn main() {
    let input = fs::read_to_string("input.md").unwrap();
    let mut tokens = VecDeque::from_iter(Tokenizer::tokenize(input));
    let md_tree = Root::parse(&mut tokens).unwrap();
    println!("{:#?}", md_tree);

    let html_tree = md_tree.to_html_node();

    println!("{}", html_tree.to_html());
}

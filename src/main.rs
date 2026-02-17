#![allow(unused)]

mod html;
mod markdown;
mod parser;
mod renderer;
mod token;
mod tokenizer;

use std::{collections::VecDeque, fs, time::Instant};

use crate::{
    html::ToHTMLNode, markdown::Root, parser::Parsable, renderer::Renderer, tokenizer::Tokenizer,
};

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input.md").unwrap();
    let mut tokens = VecDeque::from_iter(Tokenizer::tokenize(input));
    // dbg!(&tokens);
    let md_tree = Root::parse(&mut tokens).unwrap();
    // dbg!(&md_tree);

    let html_tree = md_tree.to_html_node();
    let html = Renderer::render_page(html_tree);
    fs::write("public/index.html", &html).unwrap();
    let duration = start.elapsed();

    println!("{}", html);
    println!("took {duration:?}")
}

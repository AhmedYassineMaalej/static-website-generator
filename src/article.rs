use markdown::{ParseOptions, mdast::Node};

use crate::{
    html_generator::HtmlGenerator,
    index_extractor::{Index, IndexExtractor},
    visitor::MarkdownVisitor,
};

pub struct Article {
    mdast: Node,
    index: Index,
}

impl Article {
    pub fn new(mdast: Node) -> Self {
        let Node::Root(root) = &mdast else {
            panic!("tried to build article from non-root node");
        };

        let index_extractor = IndexExtractor::new();
        let index = index_extractor.extract_index(&root);

        Self { mdast, index }
    }

    pub fn html(&self) -> String {
        let html_generator = HtmlGenerator::new();
        html_generator.generate_html(&self.mdast)
    }
}

#[test]
fn test() {
    let input = std::fs::read_to_string("input.md").unwrap();
    let ast = markdown::to_mdast(&input, &ParseOptions::default()).unwrap();
    let article = Article::new(ast);
    dbg!(article.index);
}

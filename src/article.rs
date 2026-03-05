use markdown::mdast::Node;

use crate::{
    html_generator::HtmlGenerator,
    index_extractor::{Index, IndexExtractor},
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
        let index = index_extractor.extract_index(root);

        Self { mdast, index }
    }

    pub fn html(&self) -> String {
        let html_generator = HtmlGenerator::new();
        html_generator.generate_html(&self.mdast)
    }
}

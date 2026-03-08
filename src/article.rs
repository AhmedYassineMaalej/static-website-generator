use build_html::{Html, HtmlElement, HtmlTag};
use markdown::mdast::Node;

use crate::{
    html_generator::HtmlGenerator,
    index_extractor::{Index, IndexExtractor},
};

#[derive(Debug)]
pub struct Article {
    mdast: Node,
    index: Index,
}

impl Article {
    pub fn new(mdast: Node) -> Self {
        let Node::Root(root) = &mdast else {
            panic!("tried to build article from non-root node");
        };

        let index = IndexExtractor::extract_index(root);

        Self { mdast, index }
    }

    pub fn content_html(&self) -> String {
        let html_generator = HtmlGenerator::new();
        html_generator.generate_html(&self.mdast)
    }

    pub fn index_html(&self) -> String {
        let mut element = HtmlElement::new(HtmlTag::Div);
        element.add_attribute("class", "sticky");

        for heading in &self.index {
            let mut heading_element = HtmlElement::new(HtmlTag::Link);
            heading_element.add_child(heading.into());
            heading_element.add_attribute("href", format!("#{}", heading.replace(' ', "-")));
            element.add_child(heading_element.into());
        }

        element.to_html_string()
    }
}

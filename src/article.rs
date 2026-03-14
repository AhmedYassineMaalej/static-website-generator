use build_html::{Html, HtmlElement, HtmlTag};
use markdown::mdast::Node;

use crate::{
    extractor::Extractor,
    html_generator::HtmlGenerator,
    image_extractors::{ArticleImage, ImageExtractor},
    index_extractor::{Index, IndexExtractor},
    metadata_extractor::{ArticleMetadata, MetadataExtractor},
};

#[derive(Debug)]
pub struct Article {
    mdast: Node,
    index: Index,
    pub metadata: ArticleMetadata,
    pub images: Vec<ArticleImage>,
}

impl Article {
    pub fn new(mdast: Node) -> Self {
        let Node::Root(root) = &mdast else {
            panic!("tried to build article from non-root node");
        };

        let index = IndexExtractor::extract(root);
        let images = ImageExtractor::extract(root);
        let metadata = MetadataExtractor::extract(root);

        Self {
            mdast,
            index,
            metadata,
            images,
        }
    }

    pub fn content_html(&self, debug_position: bool) -> String {
        let html_generator = HtmlGenerator::new(debug_position);
        html_generator.generate_html(&self.mdast)
    }

    pub fn index_html(&self) -> String {
        let mut element = HtmlElement::new(HtmlTag::Div);
        element.add_attribute("class", "index");
        element.add_child(
            HtmlElement::new(HtmlTag::Heading1)
                .with_child("Content".into())
                .into(),
        );

        for heading in &self.index {
            let mut heading_element = HtmlElement::new(HtmlTag::Link);
            heading_element.add_child(heading.into());
            heading_element.add_attribute("href", format!("#{}", heading.replace(' ', "-")));
            element.add_child(heading_element.into());
        }

        element.to_html_string()
    }

    pub fn title_html(&self) -> String {
        HtmlElement::new(HtmlTag::Heading1)
            .with_attribute("class", "title")
            .with_child(self.metadata.title.clone().into())
            .to_string()
    }

    pub fn tags_html(&self) -> String {
        let mut element = String::new();
        for tag in &self.metadata.tags {
            element.push_str(
                &HtmlElement::new(HtmlTag::ParagraphText)
                    .with_child(tag.clone().into())
                    .with_attribute("class", "tag")
                    .to_string(),
            );
        }

        element
    }
}

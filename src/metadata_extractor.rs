use std::time::Duration;

use toml::{Table, Value, map::Map};

use crate::{extractor::Extractor, markdown_node::MarkdownNode, markdown_visitor::MarkdownVisitor};

#[derive(Debug)]
pub struct ArticleMetadata {
    pub title: String,
    pub tags: Vec<String>,
}

#[derive(Default)]
pub struct MetadataExtractor {
    title: Option<String>,
    tags: Option<Vec<String>>,
    duration: Option<Duration>,
}

impl MetadataExtractor {
    fn from_toml_table(table: &Map<String, Value>) -> Self {
        let title = Self::extract_title(table);
        let tags = Self::extract_tags(table);
        let duration = Self::extract_duration(table);

        Self {
            title,
            tags,
            duration,
        }
    }

    fn extract_title(table: &Map<String, Value>) -> Option<String> {
        let title = table.get("title")?;

        match title {
            Value::String(title) => Some(title.clone()),
            v => {
                println!("title must string, ({v:?} found)");
                None
            }
        }
    }

    fn extract_tags(table: &Map<String, Value>) -> Option<Vec<String>> {
        let tags = table.get("tags")?;

        let Value::Array(tags) = tags else {
            println!("tags must be an array");
            return None;
        };

        let tags = tags
            .iter()
            .filter_map(|tag| match tag {
                Value::String(tag) => Some(tag.clone()),
                v => {
                    println!("tag must be string (found {v:?})");
                    None
                }
            })
            .collect();

        Some(tags)
    }

    fn extract_duration(table: &Map<String, Value>) -> Option<Duration> {
        let duration = table.get("duration")?;

        let Value::Integer(duration) = duration else {
            println!("duration must be an integer");
            return None;
        };

        let Ok(duration) = u64::try_from(*duration) else {
            println!("invalid duration: {duration} minutes");
            return None;
        };

        Some(Duration::from_mins(duration))
    }
}

impl Extractor<ArticleMetadata> for MetadataExtractor {
    fn consume(self) -> ArticleMetadata {
        ArticleMetadata {
            title: self.title.expect("article missing title"),
            tags: self.tags.expect("article missing tags"),
        }
    }
}

impl MarkdownVisitor for MetadataExtractor {
    fn visit_root(&mut self, root: &markdown::mdast::Root) {
        for node in &root.children {
            self.visit_node(node);
        }
    }

    fn visit_toml(&mut self, toml: &markdown::mdast::Toml) {
        let table = toml.value.parse::<Table>().expect("invalid toml");
        self.title = Self::extract_title(&table);
        self.tags = Self::extract_tags(&table);
        self.duration = Self::extract_duration(&table);
    }

    fn visit_blockquote(&mut self, _blockquote: &markdown::mdast::Blockquote) {}

    fn visit_footnote_definition(
        &mut self,
        _footnote_definition: &markdown::mdast::FootnoteDefinition,
    ) {
    }

    fn visit_mdx_jsx_flow_element(
        &mut self,
        _mdx_jsx_flow_element: &markdown::mdast::MdxJsxFlowElement,
    ) {
    }

    fn visit_list(&mut self, _list: &markdown::mdast::List) {}

    fn visit_mdxjs_esm(&mut self, _mdxjs_esm: &markdown::mdast::MdxjsEsm) {}

    fn visit_yaml(&mut self, _yaml: &markdown::mdast::Yaml) {}

    fn visit_break(&mut self, _break_: &markdown::mdast::Break) {}

    fn visit_inline_code(&mut self, _inline_code: &markdown::mdast::InlineCode) {}

    fn visit_inline_math(&mut self, _inline_math: &markdown::mdast::InlineMath) {}

    fn visit_delete(&mut self, _delete: &markdown::mdast::Delete) {}

    fn visit_emphasis(&mut self, _emphasis: &markdown::mdast::Emphasis) {}

    fn visit_mdx_text_expression(
        &mut self,
        _mdx_text_expression: &markdown::mdast::MdxTextExpression,
    ) {
    }

    fn visit_footnote_reference(
        &mut self,
        _footnote_reference: &markdown::mdast::FootnoteReference,
    ) {
    }

    fn visit_html(&mut self, _html: &markdown::mdast::Html) {}

    fn visit_image(&mut self, _image: &markdown::mdast::Image) {}

    fn visit_image_reference(&mut self, _image_reference: &markdown::mdast::ImageReference) {}

    fn visit_mdx_jsx_text_element(
        &mut self,
        _mdx_jsx_text_element: &markdown::mdast::MdxJsxTextElement,
    ) {
    }

    fn visit_link(&mut self, _link: &markdown::mdast::Link) {}

    fn visit_link_reference(&mut self, _link_reference: &markdown::mdast::LinkReference) {}

    fn visit_strong(&mut self, _strong: &markdown::mdast::Strong) {}

    fn visit_text(&mut self, _text: &markdown::mdast::Text) {}

    fn visit_code(&mut self, _code: &markdown::mdast::Code) {}

    fn visit_math(&mut self, _math: &markdown::mdast::Math) {}

    fn visit_mdx_flow_expression(
        &mut self,
        _mdx_flow_expression: &markdown::mdast::MdxFlowExpression,
    ) {
    }

    fn visit_heading(&mut self, _heading: &markdown::mdast::Heading) {}

    fn visit_table(&mut self, _table: &markdown::mdast::Table) {}

    fn visit_thematic_break(&mut self, _thematic_break: &markdown::mdast::ThematicBreak) {}

    fn visit_table_row(&mut self, _table_row: &markdown::mdast::TableRow) {}

    fn visit_table_cell(&mut self, _table_cell: &markdown::mdast::TableCell) {}

    fn visit_list_item(&mut self, _list_item: &markdown::mdast::ListItem) {}

    fn visit_definition(&mut self, _definition: &markdown::mdast::Definition) {}

    fn visit_paragraph(&mut self, _paragraph: &markdown::mdast::Paragraph) {}
}

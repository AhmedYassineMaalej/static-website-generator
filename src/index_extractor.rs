use markdown::mdast::{Heading, Root};

use crate::{
    extractor::Extractor, markdown_visitor::MarkdownVisitor, text_extractor::TextExtractor,
};

pub type Index = Vec<String>;

#[derive(Default)]
pub struct IndexExtractor {
    index: Index,
}

impl Extractor<Index> for IndexExtractor {
    fn consume(self) -> Index {
        self.index
    }
}

impl MarkdownVisitor<()> for IndexExtractor {
    fn visit_heading(&mut self, heading: &Heading) {
        let text = TextExtractor::extract(heading);

        self.index.push(text);
    }

    fn visit_root(&mut self, root: &Root) {
        for node in &root.children {
            self.visit_node(node);
        }
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

    fn visit_toml(&mut self, _toml: &markdown::mdast::Toml) {}

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
    fn visit_table(&mut self, _table: &markdown::mdast::Table) {}
    fn visit_thematic_break(&mut self, _thematic_break: &markdown::mdast::ThematicBreak) {}
    fn visit_table_row(&mut self, _table_row: &markdown::mdast::TableRow) {}
    fn visit_table_cell(&mut self, _table_cell: &markdown::mdast::TableCell) {}
    fn visit_list_item(&mut self, _list_item: &markdown::mdast::ListItem) {}
    fn visit_definition(&mut self, _definition: &markdown::mdast::Definition) {}
    fn visit_paragraph(&mut self, _paragraph: &markdown::mdast::Paragraph) {}
}

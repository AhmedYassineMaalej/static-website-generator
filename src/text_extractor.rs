use markdown::mdast::{Heading, Root, Text};

use crate::visitor::{MarkdownNode, MarkdownVisitor};

pub struct TextExtractor {
    text: String,
}

impl TextExtractor {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn extract_text(mut self, node: &impl MarkdownNode) -> String {
        node.accept(&mut self);
        self.text
    }
}

impl Default for TextExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownVisitor<()> for TextExtractor {
    fn visit_root(&mut self, root: &Root) {
        for node in &root.children {
            self.visit_node(node);
        }
    }

    fn visit_blockquote(&mut self, _blockquote: &markdown::mdast::Blockquote) {
        todo!()
    }

    fn visit_footnote_definition(
        &mut self,
        _footnote_definition: &markdown::mdast::FootnoteDefinition,
    ) {
        todo!()
    }

    fn visit_mdx_jsx_flow_element(
        &mut self,
        _mdx_jsx_flow_element: &markdown::mdast::MdxJsxFlowElement,
    ) {
        todo!()
    }

    fn visit_list(&mut self, _list: &markdown::mdast::List) {
        todo!()
    }

    fn visit_mdxjs_esm(&mut self, _mdxjs_esm: &markdown::mdast::MdxjsEsm) {
        todo!()
    }

    fn visit_toml(&mut self, _toml: &markdown::mdast::Toml) {
        todo!()
    }

    fn visit_yaml(&mut self, _yaml: &markdown::mdast::Yaml) {
        todo!()
    }

    fn visit_break(&mut self, _break_: &markdown::mdast::Break) {
        todo!()
    }

    fn visit_inline_code(&mut self, _inline_code: &markdown::mdast::InlineCode) {
        todo!()
    }

    fn visit_inline_math(&mut self, _inline_math: &markdown::mdast::InlineMath) {
        todo!()
    }

    fn visit_delete(&mut self, _delete: &markdown::mdast::Delete) {
        todo!()
    }

    fn visit_emphasis(&mut self, _emphasis: &markdown::mdast::Emphasis) {
        todo!()
    }

    fn visit_mdx_text_expression(
        &mut self,
        _mdx_text_expression: &markdown::mdast::MdxTextExpression,
    ) {
        todo!()
    }

    fn visit_footnote_reference(
        &mut self,
        _footnote_reference: &markdown::mdast::FootnoteReference,
    ) {
        todo!()
    }

    fn visit_html(&mut self, _html: &markdown::mdast::Html) {
        todo!()
    }

    fn visit_image(&mut self, _image: &markdown::mdast::Image) {
        todo!()
    }

    fn visit_image_reference(&mut self, _image_reference: &markdown::mdast::ImageReference) {
        todo!()
    }

    fn visit_mdx_jsx_text_element(
        &mut self,
        _mdx_jsx_text_element: &markdown::mdast::MdxJsxTextElement,
    ) {
        todo!()
    }

    fn visit_link(&mut self, _link: &markdown::mdast::Link) {
        todo!()
    }

    fn visit_link_reference(&mut self, _link_reference: &markdown::mdast::LinkReference) {
        todo!()
    }

    fn visit_strong(&mut self, _strong: &markdown::mdast::Strong) {
        todo!()
    }

    fn visit_text(&mut self, text: &Text) {
        self.text += &text.value;
    }

    fn visit_code(&mut self, _code: &markdown::mdast::Code) {
        todo!()
    }

    fn visit_math(&mut self, _math: &markdown::mdast::Math) {
        todo!()
    }

    fn visit_mdx_flow_expression(
        &mut self,
        _mdx_flow_expression: &markdown::mdast::MdxFlowExpression,
    ) {
        todo!()
    }

    fn visit_heading(&mut self, heading: &Heading) {
        for node in &heading.children {
            self.visit_node(node);
        }
    }

    fn visit_table(&mut self, _table: &markdown::mdast::Table) {
        todo!()
    }

    fn visit_thematic_break(&mut self, _thematic_break: &markdown::mdast::ThematicBreak) {
        todo!()
    }

    fn visit_table_row(&mut self, _table_row: &markdown::mdast::TableRow) {
        todo!()
    }

    fn visit_table_cell(&mut self, _table_cell: &markdown::mdast::TableCell) {
        todo!()
    }

    fn visit_list_item(&mut self, _list_item: &markdown::mdast::ListItem) {
        todo!()
    }

    fn visit_definition(&mut self, _definition: &markdown::mdast::Definition) {
        todo!()
    }

    fn visit_paragraph(&mut self, _paragraph: &markdown::mdast::Paragraph) {
        todo!()
    }
}

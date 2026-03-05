use markdown::mdast::{Heading, Node, Root, Text};

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

    fn visit_blockquote(&mut self, blockquote: &markdown::mdast::Blockquote) {
        todo!()
    }

    fn visit_footnote_definition(
        &mut self,
        footnote_definition: &markdown::mdast::FootnoteDefinition,
    ) {
        todo!()
    }

    fn visit_mdx_jsx_flow_element(
        &mut self,
        mdx_jsx_flow_element: &markdown::mdast::MdxJsxFlowElement,
    ) {
        todo!()
    }

    fn visit_list(&mut self, list: &markdown::mdast::List) {
        todo!()
    }

    fn visit_mdxjs_esm(&mut self, mdxjs_esm: &markdown::mdast::MdxjsEsm) {
        todo!()
    }

    fn visit_toml(&mut self, toml: &markdown::mdast::Toml) {
        todo!()
    }

    fn visit_yaml(&mut self, yaml: &markdown::mdast::Yaml) {
        todo!()
    }

    fn visit_break(&mut self, break_: &markdown::mdast::Break) {
        todo!()
    }

    fn visit_inline_code(&mut self, inline_code: &markdown::mdast::InlineCode) {
        todo!()
    }

    fn visit_inline_math(&mut self, inline_math: &markdown::mdast::InlineMath) {
        todo!()
    }

    fn visit_delete(&mut self, delete: &markdown::mdast::Delete) {
        todo!()
    }

    fn visit_emphasis(&mut self, emphasis: &markdown::mdast::Emphasis) {
        todo!()
    }

    fn visit_mdx_text_expression(
        &mut self,
        mdx_text_expression: &markdown::mdast::MdxTextExpression,
    ) {
        todo!()
    }

    fn visit_footnote_reference(
        &mut self,
        footnote_reference: &markdown::mdast::FootnoteReference,
    ) {
        todo!()
    }

    fn visit_html(&mut self, html: &markdown::mdast::Html) {
        todo!()
    }

    fn visit_image(&mut self, image: &markdown::mdast::Image) {
        todo!()
    }

    fn visit_image_reference(&mut self, image_reference: &markdown::mdast::ImageReference) {
        todo!()
    }

    fn visit_mdx_jsx_text_element(
        &mut self,
        mdx_jsx_text_element: &markdown::mdast::MdxJsxTextElement,
    ) {
        todo!()
    }

    fn visit_link(&mut self, link: &markdown::mdast::Link) {
        todo!()
    }

    fn visit_link_reference(&mut self, link_reference: &markdown::mdast::LinkReference) {
        todo!()
    }

    fn visit_strong(&mut self, strong: &markdown::mdast::Strong) {
        todo!()
    }

    fn visit_text(&mut self, text: &Text) {
        self.text += &text.value;
    }

    fn visit_code(&mut self, code: &markdown::mdast::Code) {
        todo!()
    }

    fn visit_math(&mut self, math: &markdown::mdast::Math) {
        todo!()
    }

    fn visit_mdx_flow_expression(
        &mut self,
        mdx_flow_expression: &markdown::mdast::MdxFlowExpression,
    ) {
        todo!()
    }

    fn visit_heading(&mut self, heading: &Heading) {
        for node in &heading.children {
            self.visit_node(node);
        }
    }

    fn visit_table(&mut self, table: &markdown::mdast::Table) {
        todo!()
    }

    fn visit_thematic_break(&mut self, thematic_break: &markdown::mdast::ThematicBreak) {
        todo!()
    }

    fn visit_table_row(&mut self, table_row: &markdown::mdast::TableRow) {
        todo!()
    }

    fn visit_table_cell(&mut self, table_cell: &markdown::mdast::TableCell) {
        todo!()
    }

    fn visit_list_item(&mut self, list_item: &markdown::mdast::ListItem) {
        todo!()
    }

    fn visit_definition(&mut self, definition: &markdown::mdast::Definition) {
        todo!()
    }

    fn visit_paragraph(&mut self, paragraph: &markdown::mdast::Paragraph) {
        todo!()
    }
}

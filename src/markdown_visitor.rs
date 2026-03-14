use markdown::mdast::{
    Blockquote, Break, Code, Definition, Delete, Emphasis, FootnoteDefinition, FootnoteReference,
    Heading, Html, Image, ImageReference, InlineCode, InlineMath, Link, LinkReference, List,
    ListItem, Math, MdxFlowExpression, MdxJsxFlowElement, MdxJsxTextElement, MdxTextExpression,
    MdxjsEsm, Node, Paragraph, Root, Strong, Table, TableCell, TableRow, Text, ThematicBreak, Toml,
    Yaml,
};

use crate::markdown_node::MarkdownNode;

#[allow(unused_variables)]
pub trait MarkdownVisitor<T = ()>: Sized {
    fn visit_node(&mut self, node: &Node) -> T {
        node.accept(self)
    }

    fn visit_root(&mut self, root: &Root) -> T;

    fn visit_blockquote(&mut self, blockquote: &Blockquote) -> T;

    fn visit_footnote_definition(&mut self, footnote_definition: &FootnoteDefinition) -> T;

    fn visit_mdx_jsx_flow_element(&mut self, mdx_jsx_flow_element: &MdxJsxFlowElement) -> T;

    fn visit_list(&mut self, list: &List) -> T;

    fn visit_mdxjs_esm(&mut self, mdxjs_esm: &MdxjsEsm) -> T;

    fn visit_toml(&mut self, toml: &Toml) -> T;

    fn visit_yaml(&mut self, yaml: &Yaml) -> T;

    fn visit_break(&mut self, break_: &Break) -> T;

    fn visit_inline_code(&mut self, inline_code: &InlineCode) -> T;

    fn visit_inline_math(&mut self, inline_math: &InlineMath) -> T;

    fn visit_delete(&mut self, delete: &Delete) -> T;

    fn visit_emphasis(&mut self, emphasis: &Emphasis) -> T;

    fn visit_mdx_text_expression(&mut self, mdx_text_expression: &MdxTextExpression) -> T;

    fn visit_footnote_reference(&mut self, footnote_reference: &FootnoteReference) -> T;

    fn visit_html(&mut self, html: &Html) -> T;

    fn visit_image(&mut self, image: &Image) -> T;

    fn visit_image_reference(&mut self, image_reference: &ImageReference) -> T;

    fn visit_mdx_jsx_text_element(&mut self, mdx_jsx_text_element: &MdxJsxTextElement) -> T;

    fn visit_link(&mut self, link: &Link) -> T;

    fn visit_link_reference(&mut self, link_reference: &LinkReference) -> T;

    fn visit_strong(&mut self, strong: &Strong) -> T;

    fn visit_text(&mut self, text: &Text) -> T;

    fn visit_code(&mut self, code: &Code) -> T;

    fn visit_math(&mut self, math: &Math) -> T;

    fn visit_mdx_flow_expression(&mut self, mdx_flow_expression: &MdxFlowExpression) -> T;

    fn visit_heading(&mut self, heading: &Heading) -> T;

    fn visit_table(&mut self, table: &Table) -> T;

    fn visit_thematic_break(&mut self, thematic_break: &ThematicBreak) -> T;

    fn visit_table_row(&mut self, table_row: &TableRow) -> T;

    fn visit_table_cell(&mut self, table_cell: &TableCell) -> T;

    fn visit_list_item(&mut self, list_item: &ListItem) -> T;

    fn visit_definition(&mut self, definition: &Definition) -> T;

    fn visit_paragraph(&mut self, paragraph: &Paragraph) -> T;
}

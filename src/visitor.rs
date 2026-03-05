use markdown::mdast::{
    Blockquote, Break, Code, Definition, Delete, Emphasis, FootnoteDefinition, FootnoteReference,
    Heading, Html, Image, ImageReference, InlineCode, InlineMath, Link, LinkReference, List,
    ListItem, Math, MdxFlowExpression, MdxJsxFlowElement, MdxJsxTextElement, MdxTextExpression,
    MdxjsEsm, Node, Paragraph, Root, Strong, Table, TableCell, TableRow, Text, ThematicBreak, Toml,
    Yaml,
};

#[allow(unused_variables)]
pub trait MarkdownVisitor<T>: Sized {
    fn visit_node(&mut self, node: &Node) -> T {
        match node {
            Node::Root(root) => self.visit_root(root),
            Node::Blockquote(blockquote) => self.visit_blockquote(blockquote),
            Node::FootnoteDefinition(footnote_definition) => {
                self.visit_footnote_definition(footnote_definition)
            }
            Node::MdxJsxFlowElement(mdx_jsx_flow_element) => {
                self.visit_mdx_jsx_flow_element(mdx_jsx_flow_element)
            }
            Node::List(list) => self.visit_list(list),
            Node::MdxjsEsm(mdxjs_esm) => self.visit_mdxjs_esm(mdxjs_esm),
            Node::Toml(toml) => self.visit_toml(toml),
            Node::Yaml(yaml) => self.visit_yaml(yaml),
            Node::Break(break_) => self.visit_break(break_),
            Node::InlineCode(inline_code) => self.visit_inline_code(inline_code),
            Node::InlineMath(inline_math) => self.visit_inline_math(inline_math),
            Node::Delete(delete) => self.visit_delete(delete),
            Node::Emphasis(emphasis) => self.visit_emphasis(emphasis),
            Node::MdxTextExpression(mdx_text_expression) => {
                self.visit_mdx_text_expression(mdx_text_expression)
            }
            Node::FootnoteReference(footnote_reference) => {
                self.visit_footnote_reference(footnote_reference)
            }
            Node::Html(html) => self.visit_html(html),
            Node::Image(image) => self.visit_image(image),
            Node::ImageReference(image_reference) => self.visit_image_reference(image_reference),
            Node::MdxJsxTextElement(mdx_jsx_text_element) => {
                self.visit_mdx_jsx_text_element(mdx_jsx_text_element)
            }
            Node::Link(link) => self.visit_link(link),
            Node::LinkReference(link_reference) => self.visit_link_reference(link_reference),
            Node::Strong(strong) => self.visit_strong(strong),
            Node::Text(text) => self.visit_text(text),
            Node::Code(code) => self.visit_code(code),
            Node::Math(math) => self.visit_math(math),
            Node::MdxFlowExpression(mdx_flow_expression) => {
                self.visit_mdx_flow_expression(mdx_flow_expression)
            }
            Node::Heading(heading) => heading.accept(self),
            Node::Table(table) => self.visit_table(table),
            Node::ThematicBreak(thematic_break) => self.visit_thematic_break(thematic_break),
            Node::TableRow(table_row) => self.visit_table_row(table_row),
            Node::TableCell(table_cell) => self.visit_table_cell(table_cell),
            Node::ListItem(list_item) => self.visit_list_item(list_item),
            Node::Definition(definition) => self.visit_definition(definition),
            Node::Paragraph(paragraph) => self.visit_paragraph(paragraph),
        }
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

pub trait MarkdownNode {
    fn accept<T>(&self, visitor: &mut impl MarkdownVisitor<T>) -> T;
}

impl MarkdownNode for Heading {
    fn accept<T>(&self, visitor: &mut impl MarkdownVisitor<T>) -> T {
        visitor.visit_heading(self)
    }
}

impl MarkdownNode for Root {
    fn accept<T>(&self, visitor: &mut impl MarkdownVisitor<T>) -> T {
        visitor.visit_root(self)
    }
}

impl MarkdownNode for Code {
    fn accept<T>(&self, visitor: &mut impl MarkdownVisitor<T>) -> T {
        visitor.visit_code(self)
    }
}

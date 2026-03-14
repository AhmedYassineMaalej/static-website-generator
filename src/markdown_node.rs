use markdown::mdast::{
    Blockquote, Break, Code, Definition, Delete, Emphasis, FootnoteDefinition, FootnoteReference,
    Heading, Html, Image, ImageReference, InlineCode, InlineMath, Link, LinkReference, List,
    ListItem, Math, MdxFlowExpression, MdxJsxFlowElement, MdxJsxTextElement, MdxTextExpression,
    MdxjsEsm, Node, Paragraph, Root, Strong, Table, TableCell, TableRow, Text, ThematicBreak, Toml,
    Yaml,
};

use crate::markdown_visitor::MarkdownVisitor;

pub trait MarkdownNode {
    fn accept<T>(&self, visitor: &mut impl MarkdownVisitor<T>) -> T;
}

impl MarkdownNode for Node {
    fn accept<T>(&self, visitor: &mut impl MarkdownVisitor<T>) -> T {
        match self {
            Node::Root(root) => root.accept(visitor),
            Node::Blockquote(blockquote) => blockquote.accept(visitor),
            Node::FootnoteDefinition(footnote_definition) => footnote_definition.accept(visitor),
            Node::MdxJsxFlowElement(mdx_jsx_flow_element) => mdx_jsx_flow_element.accept(visitor),
            Node::List(list) => list.accept(visitor),
            Node::MdxjsEsm(mdxjs_esm) => mdxjs_esm.accept(visitor),
            Node::Toml(toml) => toml.accept(visitor),
            Node::Yaml(yaml) => yaml.accept(visitor),
            Node::Break(bbreak) => bbreak.accept(visitor),
            Node::InlineCode(inline_code) => inline_code.accept(visitor),
            Node::InlineMath(inline_math) => inline_math.accept(visitor),
            Node::Delete(delete) => delete.accept(visitor),
            Node::Emphasis(emphasis) => emphasis.accept(visitor),
            Node::MdxTextExpression(mdx_text_expression) => mdx_text_expression.accept(visitor),
            Node::FootnoteReference(footnote_reference) => footnote_reference.accept(visitor),
            Node::Html(html) => html.accept(visitor),
            Node::Image(image) => image.accept(visitor),
            Node::ImageReference(image_reference) => image_reference.accept(visitor),
            Node::MdxJsxTextElement(mdx_jsx_text_element) => mdx_jsx_text_element.accept(visitor),
            Node::Link(link) => link.accept(visitor),
            Node::LinkReference(link_reference) => link_reference.accept(visitor),
            Node::Strong(strong) => strong.accept(visitor),
            Node::Text(text) => text.accept(visitor),
            Node::Code(code) => code.accept(visitor),
            Node::Math(math) => math.accept(visitor),
            Node::MdxFlowExpression(mdx_flow_expression) => mdx_flow_expression.accept(visitor),
            Node::Heading(heading) => heading.accept(visitor),
            Node::Table(table) => table.accept(visitor),
            Node::ThematicBreak(thematic_break) => thematic_break.accept(visitor),
            Node::TableRow(table_row) => table_row.accept(visitor),
            Node::TableCell(table_cell) => table_cell.accept(visitor),
            Node::ListItem(list_item) => list_item.accept(visitor),
            Node::Definition(definition) => definition.accept(visitor),
            Node::Paragraph(paragraph) => paragraph.accept(visitor),
        }
    }
}

macro_rules! implement_markdown_node {
    ($type: ty, $name: ident) => {
        impl MarkdownNode for $type {
            fn accept<T>(&self, visitor: &mut impl MarkdownVisitor<T>) -> T {
                visitor.$name(self)
            }
        }
    };
}

implement_markdown_node!(Root, visit_root);
implement_markdown_node!(Blockquote, visit_blockquote);
implement_markdown_node!(FootnoteDefinition, visit_footnote_definition);
implement_markdown_node!(MdxJsxFlowElement, visit_mdx_jsx_flow_element);
implement_markdown_node!(List, visit_list);
implement_markdown_node!(MdxjsEsm, visit_mdxjs_esm);
implement_markdown_node!(Toml, visit_toml);
implement_markdown_node!(Yaml, visit_yaml);
implement_markdown_node!(Break, visit_break);
implement_markdown_node!(InlineCode, visit_inline_code);
implement_markdown_node!(InlineMath, visit_inline_math);
implement_markdown_node!(Delete, visit_delete);
implement_markdown_node!(Emphasis, visit_emphasis);
implement_markdown_node!(MdxTextExpression, visit_mdx_text_expression);
implement_markdown_node!(FootnoteReference, visit_footnote_reference);
implement_markdown_node!(Html, visit_html);
implement_markdown_node!(Image, visit_image);
implement_markdown_node!(ImageReference, visit_image_reference);
implement_markdown_node!(MdxJsxTextElement, visit_mdx_jsx_text_element);
implement_markdown_node!(Link, visit_link);
implement_markdown_node!(LinkReference, visit_link_reference);
implement_markdown_node!(Strong, visit_strong);
implement_markdown_node!(Text, visit_text);
implement_markdown_node!(Code, visit_code);
implement_markdown_node!(Math, visit_math);
implement_markdown_node!(MdxFlowExpression, visit_mdx_flow_expression);
implement_markdown_node!(Heading, visit_heading);
implement_markdown_node!(Table, visit_table);
implement_markdown_node!(ThematicBreak, visit_thematic_break);
implement_markdown_node!(TableRow, visit_table_row);
implement_markdown_node!(TableCell, visit_table_cell);
implement_markdown_node!(ListItem, visit_list_item);
implement_markdown_node!(Definition, visit_definition);
implement_markdown_node!(Paragraph, visit_paragraph);

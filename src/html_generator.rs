use build_html::{HtmlChild, HtmlElement, HtmlTag};
use markdown::{
    ParseOptions,
    mdast::{Node, Root},
    unist::Point,
};

use crate::visitor::{MarkdownNode, MarkdownVisitor};

pub struct HtmlGenerator {}

impl HtmlGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_html(mut self, node: &Node) -> String {
        let root = self.visit_node(node);
        root.to_string()
    }
}

impl MarkdownVisitor<HtmlChild> for HtmlGenerator {
    fn visit_root(&mut self, root: &Root) -> HtmlChild {
        let mut element = HtmlElement::new(HtmlTag::Div);
        for node in &root.children {
            element.add_child(self.visit_node(node));
        }

        element.into()
    }

    fn visit_heading(&mut self, heading: &markdown::mdast::Heading) -> HtmlChild {
        let tag = match heading.depth {
            1 => HtmlTag::Heading1,
            2 => HtmlTag::Heading2,
            3 => HtmlTag::Heading3,
            4 => HtmlTag::Heading4,
            5 => HtmlTag::Heading5,
            6 => HtmlTag::Heading6,
            n => unreachable!("heading with depth: {n}"),
        };

        let mut element = HtmlElement::new(tag);
        for node in &heading.children {
            element.add_child(self.visit_node(node));
        }

        element.into()
    }

    fn visit_paragraph(&mut self, paragraph: &markdown::mdast::Paragraph) -> HtmlChild {
        let mut element = HtmlElement::new(HtmlTag::ParagraphText);
        for node in &paragraph.children {
            element.add_child(self.visit_node(node));
        }

        element.into()
    }

    fn visit_blockquote(&mut self, blockquote: &markdown::mdast::Blockquote) -> HtmlChild {
        todo!()
    }

    fn visit_footnote_definition(
        &mut self,
        footnote_definition: &markdown::mdast::FootnoteDefinition,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_mdx_jsx_flow_element(
        &mut self,
        mdx_jsx_flow_element: &markdown::mdast::MdxJsxFlowElement,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_list(&mut self, list: &markdown::mdast::List) -> HtmlChild {
        todo!()
    }

    fn visit_mdxjs_esm(&mut self, mdxjs_esm: &markdown::mdast::MdxjsEsm) -> HtmlChild {
        todo!()
    }

    fn visit_toml(&mut self, toml: &markdown::mdast::Toml) -> HtmlChild {
        todo!()
    }

    fn visit_yaml(&mut self, yaml: &markdown::mdast::Yaml) -> HtmlChild {
        todo!()
    }

    fn visit_break(&mut self, break_: &markdown::mdast::Break) -> HtmlChild {
        todo!()
    }

    fn visit_inline_code(&mut self, inline_code: &markdown::mdast::InlineCode) -> HtmlChild {
        todo!()
    }

    fn visit_inline_math(&mut self, inline_math: &markdown::mdast::InlineMath) -> HtmlChild {
        todo!()
    }

    fn visit_delete(&mut self, delete: &markdown::mdast::Delete) -> HtmlChild {
        todo!()
    }

    fn visit_emphasis(&mut self, emphasis: &markdown::mdast::Emphasis) -> HtmlChild {
        todo!()
    }

    fn visit_mdx_text_expression(
        &mut self,
        mdx_text_expression: &markdown::mdast::MdxTextExpression,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_footnote_reference(
        &mut self,
        footnote_reference: &markdown::mdast::FootnoteReference,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_html(&mut self, html: &markdown::mdast::Html) -> HtmlChild {
        todo!()
    }

    fn visit_image(&mut self, image: &markdown::mdast::Image) -> HtmlChild {
        todo!()
    }

    fn visit_image_reference(
        &mut self,
        image_reference: &markdown::mdast::ImageReference,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_mdx_jsx_text_element(
        &mut self,
        mdx_jsx_text_element: &markdown::mdast::MdxJsxTextElement,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_link(&mut self, link: &markdown::mdast::Link) -> HtmlChild {
        todo!()
    }

    fn visit_link_reference(
        &mut self,
        link_reference: &markdown::mdast::LinkReference,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_strong(&mut self, strong: &markdown::mdast::Strong) -> HtmlChild {
        todo!()
    }

    fn visit_text(&mut self, text: &markdown::mdast::Text) -> HtmlChild {
        let Point { line, column, .. } = text.position.clone().unwrap().start;
        let mut html = String::new();
        let text = &text.value;
        let mut offset = 0;
        for word in text.split(' ') {
            let mut word_span = HtmlElement::new(HtmlTag::Span);
            word_span.add_attribute("class", "word");

            for char in word.chars() {
                let mut char_span = HtmlElement::new(HtmlTag::Span);
                char_span.add_child(HtmlChild::Raw(char.to_string()));
                char_span.add_attribute("class", "letter");
                char_span.add_attribute("data-position", format!("{}:{}", line, column + offset));

                word_span.add_child(HtmlChild::Element(char_span));
                offset += 1;
            }
            html += &word_span.to_string();
            html += "&nbsp;";
            offset += 1;
        }

        html.into()
    }

    fn visit_code(&mut self, code: &markdown::mdast::Code) -> HtmlChild {
        todo!()
    }

    fn visit_math(&mut self, math: &markdown::mdast::Math) -> HtmlChild {
        todo!()
    }

    fn visit_mdx_flow_expression(
        &mut self,
        mdx_flow_expression: &markdown::mdast::MdxFlowExpression,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_table(&mut self, table: &markdown::mdast::Table) -> HtmlChild {
        todo!()
    }

    fn visit_thematic_break(
        &mut self,
        thematic_break: &markdown::mdast::ThematicBreak,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_table_row(&mut self, table_row: &markdown::mdast::TableRow) -> HtmlChild {
        todo!()
    }

    fn visit_table_cell(&mut self, table_cell: &markdown::mdast::TableCell) -> HtmlChild {
        todo!()
    }

    fn visit_list_item(&mut self, list_item: &markdown::mdast::ListItem) -> HtmlChild {
        todo!()
    }

    fn visit_definition(&mut self, definition: &markdown::mdast::Definition) -> HtmlChild {
        todo!()
    }

    fn visit_node(&mut self, node: &Node) -> HtmlChild {
        // TODO: replace all this with node.accept(...)
        let element = match node {
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
        };

        let Some(position) = node.position() else {
            return element;
        };

        let HtmlChild::Element(mut element) = element else {
            return element;
        };

        let line = position.start.line;
        let column = position.start.column;
        element.add_attribute("data-position", format!("{line}:{column}"));
        element.into()
    }
}

#[test]
fn test_html() {
    let input = std::fs::read_to_string("input.md").unwrap();
    let ast = markdown::to_mdast(&input, &ParseOptions::default()).unwrap();
    let html_generator = HtmlGenerator::new();
    let html = html_generator.generate_html(&ast);
    dbg!(&html);

    assert_eq!(html, markdown::to_html(&input).replace('\n', ""));
}


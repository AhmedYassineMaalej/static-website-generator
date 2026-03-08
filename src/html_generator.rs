use build_html::{HtmlChild, HtmlElement, HtmlTag};
use markdown::{
    mdast::{Node, Root},
    unist::Point,
};

use crate::{
    highlight::Highlighter,
    text_extractor::TextExtractor,
    visitor::{MarkdownNode, MarkdownVisitor},
};

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
        let heading_text = TextExtractor::extract_text(heading);
        element.add_attribute("id", heading_text.replace(' ', "-"));

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

    fn visit_blockquote(&mut self, _blockquote: &markdown::mdast::Blockquote) -> HtmlChild {
        todo!()
    }

    fn visit_footnote_definition(
        &mut self,
        _footnote_definition: &markdown::mdast::FootnoteDefinition,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_mdx_jsx_flow_element(
        &mut self,
        _mdx_jsx_flow_element: &markdown::mdast::MdxJsxFlowElement,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_list(&mut self, _list: &markdown::mdast::List) -> HtmlChild {
        todo!()
    }

    fn visit_mdxjs_esm(&mut self, _mdxjs_esm: &markdown::mdast::MdxjsEsm) -> HtmlChild {
        todo!()
    }

    fn visit_toml(&mut self, _toml: &markdown::mdast::Toml) -> HtmlChild {
        todo!()
    }

    fn visit_yaml(&mut self, yaml: &markdown::mdast::Yaml) -> HtmlChild {
        let yaml = yaml.value.clone();

        let (key, value) = yaml.split_once(':').unwrap();
        assert_eq!(key, "title");

        let mut element = HtmlElement::new(HtmlTag::Heading1);
        element.add_child(value.into());
        element.into()
    }

    fn visit_break(&mut self, _break_: &markdown::mdast::Break) -> HtmlChild {
        todo!()
    }

    fn visit_inline_code(&mut self, _inline_code: &markdown::mdast::InlineCode) -> HtmlChild {
        todo!()
    }

    fn visit_inline_math(&mut self, _inline_math: &markdown::mdast::InlineMath) -> HtmlChild {
        todo!()
    }

    fn visit_delete(&mut self, _delete: &markdown::mdast::Delete) -> HtmlChild {
        todo!()
    }

    fn visit_emphasis(&mut self, _emphasis: &markdown::mdast::Emphasis) -> HtmlChild {
        todo!()
    }

    fn visit_mdx_text_expression(
        &mut self,
        _mdx_text_expression: &markdown::mdast::MdxTextExpression,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_footnote_reference(
        &mut self,
        _footnote_reference: &markdown::mdast::FootnoteReference,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_html(&mut self, _html: &markdown::mdast::Html) -> HtmlChild {
        todo!()
    }

    fn visit_image(&mut self, _image: &markdown::mdast::Image) -> HtmlChild {
        todo!()
    }

    fn visit_image_reference(
        &mut self,
        _image_reference: &markdown::mdast::ImageReference,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_mdx_jsx_text_element(
        &mut self,
        _mdx_jsx_text_element: &markdown::mdast::MdxJsxTextElement,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_link(&mut self, link: &markdown::mdast::Link) -> HtmlChild {
        let mut element = HtmlElement::new(HtmlTag::Link);
        element.add_attribute("href", &link.url);
        for child in &link.children {
            element.add_child(self.visit_node(child));
        }

        element.into()
    }

    fn visit_link_reference(
        &mut self,
        _link_reference: &markdown::mdast::LinkReference,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_strong(&mut self, _strong: &markdown::mdast::Strong) -> HtmlChild {
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
        let lang = code.lang.as_ref().unwrap();
        let start = code.position.clone().unwrap().start;
        let (line, column) = (start.line + 1, 1); // put cursor at code start

        Highlighter::highlight(&code.value, lang, (line, column)).into()
    }

    fn visit_math(&mut self, _math: &markdown::mdast::Math) -> HtmlChild {
        todo!()
    }

    fn visit_mdx_flow_expression(
        &mut self,
        _mdx_flow_expression: &markdown::mdast::MdxFlowExpression,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_table(&mut self, _table: &markdown::mdast::Table) -> HtmlChild {
        todo!()
    }

    fn visit_thematic_break(
        &mut self,
        _thematic_break: &markdown::mdast::ThematicBreak,
    ) -> HtmlChild {
        todo!()
    }

    fn visit_table_row(&mut self, _table_row: &markdown::mdast::TableRow) -> HtmlChild {
        todo!()
    }

    fn visit_table_cell(&mut self, _table_cell: &markdown::mdast::TableCell) -> HtmlChild {
        todo!()
    }

    fn visit_list_item(&mut self, _list_item: &markdown::mdast::ListItem) -> HtmlChild {
        todo!()
    }

    fn visit_definition(&mut self, _definition: &markdown::mdast::Definition) -> HtmlChild {
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

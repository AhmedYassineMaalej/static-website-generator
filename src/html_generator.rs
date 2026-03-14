use build_html::{HtmlChild, HtmlElement, HtmlTag};
use markdown::{
    mdast::{Node, Root},
    unist::Point,
};

use crate::{
    extractor::Extractor, highlight::Highlighter, markdown_node::MarkdownNode,
    markdown_visitor::MarkdownVisitor, text_extractor::TextExtractor,
};

pub struct HtmlGenerator {
    debug_positions: bool,
}

impl HtmlGenerator {
    pub fn new(debug_positions: bool) -> Self {
        Self { debug_positions }
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
        let heading_text = TextExtractor::extract(heading);
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
        "".into()
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

    fn visit_image(&mut self, image: &markdown::mdast::Image) -> HtmlChild {
        let mut element = HtmlElement::new(HtmlTag::Image);
        element.add_attribute("alt", &image.alt);
        element.add_attribute("src", &image.url);

        element.into()
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
        element.add_attribute("target", "_blank");
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
        if !self.debug_positions {
            return text.value.clone().into();
        }

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

    fn visit_math(&mut self, math: &markdown::mdast::Math) -> HtmlChild {
        let mut element = HtmlElement::new(HtmlTag::ParagraphText);

        element.add_child(format!("$$\n{}\n$$", math.value.clone()).into());

        element.into()
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
        let element = node.accept(self);
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

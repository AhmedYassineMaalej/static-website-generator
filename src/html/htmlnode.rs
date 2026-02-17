use super::{
    Body, Code, Emphasis, Heading, InlineCode, Link, Paragraph, Properties, Strong, ToHtml,
};

pub trait ToHTMLNode {
    fn to_html_node(self) -> HTMLNode;
}

pub enum HTMLNode {
    Body(Body),
    Text(String),
    Strong(Strong),
    Emphasis(Emphasis),
    Heading(Heading),
    InlineCode(InlineCode),
    Break,
    Link(Link),
    Paragraph(Paragraph),
    Code(Code),
}

impl ToHtml for HTMLNode {
    fn to_html(&self) -> String {
        match self {
            HTMLNode::Text(str) => str.clone(),
            HTMLNode::Strong(strong) => strong.to_html(),
            HTMLNode::Emphasis(emphasis) => emphasis.to_html(),
            HTMLNode::Break => String::from("<br>"),
            HTMLNode::Heading(heading) => heading.to_html(),
            HTMLNode::InlineCode(inline_code) => inline_code.to_html(),
            HTMLNode::Body(body) => body.to_html(),
            HTMLNode::Link(link) => link.to_html(),
            HTMLNode::Paragraph(paragraph) => paragraph.to_html(),
            HTMLNode::Code(code) => code.to_html(),
        }
    }
}

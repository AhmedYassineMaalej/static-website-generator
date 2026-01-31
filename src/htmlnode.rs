use crate::properties::{Properties, ToHtml};

pub trait HTMLNode {
    fn tag(&self) -> &String;
    fn value(&self) -> Option<&String> {
        None
    }
    fn props(&self) -> &Properties;
    fn children(&self) -> Option<&Vec<Box<dyn ToHtml>>> {
        None
    }
}

impl<T> ToHtml for T
where
    T: ?Sized + HTMLNode,
{
    fn to_html(&self) -> String {
        let tag = self.tag();

        let props = self.props();
        let children = self.children();
        let value = match self.value() {
            Some(v) => v,
            None => &String::new(),
        };

        match children {
            Some(children) => {
                if props.is_empty() {
                    format!("<{tag}>{}</{tag}>", children.to_html())
                } else {
                    format!("<{tag} {props}>{value}</{tag}>", props = props.to_html())
                }
            }
            None => {
                if props.is_empty() {
                    format!("<{tag}>{value}</{tag}>")
                } else {
                    format!("<{tag} {props}>{value}</{tag}>", props = props.to_html())
                }
            }
        }
    }
}

impl<T: AsRef<dyn ToHtml>> ToHtml for &Vec<T> {
    fn to_html(&self) -> String {
        self.iter()
            .map(|node| node.as_ref().to_html())
            .reduce(|a, b| a + &b)
            .unwrap_or_default()
    }
}

impl ToHtml for String {
    fn to_html(&self) -> String {
        self.clone()
    }
}

impl ToHtml for &str {
    fn to_html(&self) -> String {
        String::from(*self)
    }
}

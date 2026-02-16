use crate::properties::{Properties, ToHtml};

pub trait ToHTMLNode {
    fn to_html_node(self) -> Box<dyn HTMLNode>;
}

pub trait HTMLNode {
    fn tag(&self) -> Option<&String>;
    fn value(&self) -> Option<&String> {
        None
    }
    fn props(&self) -> &Properties;
    fn children(&self) -> Option<&Vec<Box<dyn HTMLNode>>> {
        None
    }
}

impl<T> ToHtml for T
where
    T: ?Sized + HTMLNode,
{
    fn to_html(&self) -> String {
        let Some(tag) = self.tag() else {
            return self.value().unwrap().clone();
        };

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

impl<T: AsRef<dyn HTMLNode>> ToHtml for &Vec<T> {
    fn to_html(&self) -> String {
        self.iter()
            .map(|node| node.as_ref().to_html())
            .reduce(|a, b| a + &b)
            .unwrap_or_default()
    }
}

impl HTMLNode for String {
    fn tag(&self) -> Option<&String> {
        None
    }

    fn value(&self) -> Option<&String> {
        Some(self)
    }

    fn props(&self) -> &Properties {
        todo!()
    }
}

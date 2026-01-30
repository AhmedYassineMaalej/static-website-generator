use std::fmt::Debug;

use crate::properties::{Properties, ToHtml};

pub trait HTMLNode: Debug + Default {
    fn tag(&self) -> String;
    fn value(&self) -> Option<String>;
    fn props(&self) -> Properties;
    fn children(&self) -> Option<Vec<impl HTMLNode>>;
    fn to_html(&self) -> String {
        let tag = self.tag();
        let props = self.props();
        let children = self.children();
        let Some(value) = self.value() else {
            unimplemented!();
        };

        match children {
            Some(_) => unimplemented!(),
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

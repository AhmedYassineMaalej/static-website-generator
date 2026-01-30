use std::{collections::HashMap, fmt::Debug};

pub trait HTMLNode: Debug + Default {
    fn tag(&self) -> String;
    fn to_html(&self) -> String;
    fn props(&self) -> String;
    fn children(&self) -> Vec<impl HTMLNode>;
}

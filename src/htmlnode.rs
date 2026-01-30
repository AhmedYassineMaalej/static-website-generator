use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct HTMLNode {
    tag: Option<String>,
    value: Option<String>,
    children: Vec<HTMLNode>,
    props: HashMap<String, String>,
}

impl HTMLNode {
    fn to_html(&self) -> String {
        todo!()
    }

    fn props_to_html(&self) -> String {
        self.props
            .iter()
            .map(|(prop, val)| format!("{prop}={val:?}"))
            .reduce(|a, b| a + " " + &b)
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_props_empty() {
        let node = HTMLNode {
            props: HashMap::new(),
            ..Default::default()
        };

        assert_eq!(node.props_to_html(), String::new());
    }

    #[test]
    fn test_props_single() {
        let mut props = HashMap::new();
        props.insert(String::from("href"), String::from("https://www.google.com"));

        let node = HTMLNode {
            props,
            ..Default::default()
        };

        assert_eq!(node.props_to_html(), "href=\"https://www.google.com\"");
    }
}

use std::collections::HashMap;

pub type Properties = HashMap<String, String>;

pub trait ToHtml {
    fn to_html(&self) -> String;
}

impl ToHtml for Properties {
    fn to_html(&self) -> String {
        self.iter()
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
        let props = Properties::new();

        assert_eq!(props.to_html(), String::new());
    }

    #[test]
    fn test_props_single() {
        let mut props = Properties::new();
        props.insert(String::from("href"), String::from("https://www.google.com"));

        assert_eq!(props.to_html(), "href=\"https://www.google.com\"");
    }

    #[test]
    fn test_props_multiple() {
        let mut props = Properties::new();
        props.insert(String::from("href"), String::from("https://www.google.com"));
        props.insert(String::from("target"), String::from("_blank"));

        assert!(
            [
                "href=\"https://www.google.com\" target=\"_blank\"",
                "target=\"_blank\" href=\"https://www.google.com\""
            ]
            .contains(&props.to_html().as_str())
        );
    }
}

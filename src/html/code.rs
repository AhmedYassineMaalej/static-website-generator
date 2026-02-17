use crate::html::ToHtml;

pub struct Code {
    pub code: String,
    pub language: Option<String>,
}

impl ToHtml for Code {
    fn to_html(&self) -> String {
        format!("<code>{}</code>", self.code)
    }
}

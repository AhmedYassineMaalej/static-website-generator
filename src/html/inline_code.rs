use super::ToHtml;

pub struct InlineCode {
    pub code: String,
}

impl ToHtml for InlineCode {
    fn to_html(&self) -> String {
        format!("<code>{}</code>", self.code)
    }
}

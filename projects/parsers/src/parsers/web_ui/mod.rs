use crate::DiffuserPrompts;

impl DiffuserPrompts {
    pub fn parse_web_ui<S: AsRef<str>>(prompt: S) -> Self {
        Self { tags: vec![prompt.as_ref().to_string()] }
    }
}

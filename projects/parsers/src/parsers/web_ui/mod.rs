use crate::DiffuserPrompts;

impl DiffuserPrompts {
    pub fn from_web_ui(prompt: &str) -> Self {
        Self { tags: vec![prompt.to_string()] }
    }
}

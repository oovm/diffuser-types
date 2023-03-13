use crate::DiffuserPrompts;

impl DiffuserPrompts {
    pub fn from_novel_ai(prompt: &str) -> Self {
        Self { tags: vec![prompt.to_string()] }
    }
}

use crate::DiffuserPrompts;

impl DiffuserPrompts {
    pub fn parse_novel_ai<S: AsRef<str>>(prompt: S) -> Self {
        Self { tags: vec![prompt.as_ref().to_string()] }
    }
}

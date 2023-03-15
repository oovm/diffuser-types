use std::str::FromStr;

use crate::DiffuserPrompts;

mod novel_ai;
mod web_ui;

impl DiffuserPrompts {
    pub fn parse<S: AsRef<str>>(prompt: S) -> Self {
        Self { tags: vec![prompt.as_ref().to_string()] }
    }
}

impl<I, S> From<I> for DiffuserPrompts
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    fn from(value: I) -> Self {
        Self { tags: value.into_iter().map(parse_one).collect() }
    }
}

impl FromStr for DiffuserPrompts {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = DiffuserPrompts::default();
        for i in s.lines() {
            out.parse_by_line(i.trim())
        }
        Ok(out)
    }
}

impl DiffuserPrompts {
    fn parse_by_line(&mut self, line: &str) {
        if line.starts_with('#') {
            return;
        }
        for part in line.split(',') {
            self.tags.push(parse_one(part))
        }
    }
}

fn parse_one<S: AsRef<str>>(s: S) -> String {
    s.as_ref().trim().to_string()
}

use std::str::FromStr;

use crate::DiffuserPrompts;

mod novel_ai;
mod web_ui;

impl<I, S> From<I> for DiffuserPrompts
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    fn from(value: I) -> Self {
        Self { tags: value.into_iter().map(|s| s.as_ref().trim().to_string()).collect() }
    }
}

impl FromStr for DiffuserPrompts {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

fn parse_one<S: AsRef<str>>(s: S) -> String {
    s.as_ref().trim().to_string()
}

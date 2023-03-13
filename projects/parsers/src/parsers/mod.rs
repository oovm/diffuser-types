use std::str::FromStr;

use crate::DiffuserPrompts;

mod novel_ai;
mod web_ui;

impl FromStr for DiffuserPrompts {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl<I> From<I> for DiffuserPrompts
where
    I: IntoIterator<Item = impl Into<String>>,
{
    fn from(value: I) -> Self {
        Self { tags: value.into_iter().map(|s| s.into()).collect() }
    }
}

#[test]
fn test() {
    let prompts = DiffuserPrompts::from(vec!["a", "b", "c"]);
    println!("{:?}", prompts)
}

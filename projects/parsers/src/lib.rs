mod parsers;

mod third_party;

/// A struct that holds the prompts for the image generation.
#[derive(Clone, Debug, Default)]
pub struct DiffuserPrompts {
    tags: Vec<String>,
}

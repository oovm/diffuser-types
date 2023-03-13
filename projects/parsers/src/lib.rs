use serde::{Deserialize, Serialize};

mod parsers;

mod third_party;

/// A struct that holds the prompts for the image generation.
#[derive(Debug, Clone)]
pub struct DiffuserPrompts {
    tags: Vec<String>,
}

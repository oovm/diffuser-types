use std::{fmt::Formatter, str::FromStr};

use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::DiffuserPrompts;

struct PromptVisitor<'i> {
    tags: &'i mut Vec<String>,
}

impl<'de> Deserialize<'de> for DiffuserPrompts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut out = vec![];
        deserializer.deserialize_any(PromptVisitor { tags: &mut out })?;
        Ok(DiffuserPrompts { tags: out })
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PromptVisitor { tags: &mut place.tags })?;
        Ok(())
    }
}

impl<'de> Visitor<'de> for PromptVisitor<'_> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("except sequence of strings")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match DiffuserPrompts::from_str(v) {
            Ok(prompts) => {
                self.tags.extend(prompts.tags);
                Ok(())
            }
            Err(_) => Err(Error::custom("invalid prompts")),
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        while let Some(value) = seq.next_element()? {
            self.tags.push(value);
        }
        Ok(())
    }
}

use serde::{
    ser::{SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};

use crate::DiffuserPrompts;

impl Serialize for DiffuserPrompts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_seq(Some(self.tags.len()))?;
        for tag in &self.tags {
            s.serialize_element(tag)?;
        }
        s.end()
    }
}

use std::{fmt::Formatter, mem::transmute};

use serde::{
    __private::de::{ContentDeserializer, TaggedContentVisitor},
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::{DDIMScheduler, DiffuserScheduler, DiffuserSchedulerKind, EulerScheduler};

struct SchedulerVisitor {}

impl<'de> Deserialize<'de> for DiffuserScheduler {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let tagged = deserializer.deserialize_any(TaggedContentVisitor::<DiffuserSchedulerKind>::new(
            "type",
            "internally tagged enum DiffuserScheduler",
        ))?;
        match tagged.tag {
            DiffuserSchedulerKind::Euler => {
                let scheduler = EulerScheduler::deserialize(ContentDeserializer::<D::Error>::new(tagged.content))?;
                Ok(DiffuserScheduler::Euler(Box::new(scheduler)))
            }
            DiffuserSchedulerKind::DDIM => {
                let scheduler = DDIMScheduler::deserialize(ContentDeserializer::<D::Error>::new(tagged.content))?;
                Ok(DiffuserScheduler::DDIM(Box::new(scheduler)))
            }
        }
    }
}

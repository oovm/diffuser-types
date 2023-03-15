use std::{fmt::Formatter, str::FromStr};

use serde::{
    __private::de::{ContentDeserializer, TaggedContentVisitor},
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::{DDIMScheduler, DiffuserScheduler, DiffuserSchedulerKind, EulerDiscreteScheduler};

/// should support config from [huggingface/schedulers](https://github.com/huggingface/diffusers/tree/main/src/diffusers/schedulers)
struct SchedulerDeserializer;

impl<'de> Deserialize<'de> for DiffuserScheduler {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SchedulerDeserializer)
    }
}

impl<'de> Visitor<'de> for SchedulerDeserializer {
    type Value = DiffuserScheduler;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Except scheduler name or Scheduler object.")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match DiffuserSchedulerKind::from_str(v) {
            Ok(o) => Ok(o.as_scheduler()),
            Err(e) => Err(Error::custom(e)),
        }
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let visitor = TaggedContentVisitor::<DiffuserSchedulerKind>::new("type", "internally tagged enum DiffuserScheduler");
        let tagged = visitor.visit_map(map)?;
        match tagged.tag {
            DiffuserSchedulerKind::EulerDiscrete => {
                let scheduler = EulerDiscreteScheduler::deserialize(ContentDeserializer::<A::Error>::new(tagged.content))?;
                Ok(DiffuserScheduler::Euler(Box::new(scheduler)))
            }
            DiffuserSchedulerKind::DDIM => {
                let scheduler = DDIMScheduler::deserialize(ContentDeserializer::<A::Error>::new(tagged.content))?;
                Ok(DiffuserScheduler::DDIM(Box::new(scheduler)))
            }
        }
    }
}

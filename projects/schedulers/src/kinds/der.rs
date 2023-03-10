use std::{fmt::Formatter, mem::transmute, str::FromStr};

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer,
};

use crate::DiffuserSchedulerKind;

struct SchedulerKindDeserializer;

impl<'de> Deserialize<'de> for DiffuserSchedulerKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SchedulerKindDeserializer)
    }
}

impl FromStr for DiffuserSchedulerKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("ddim") {
            Ok(DiffuserSchedulerKind::DDIM)
        }
        else if s.eq_ignore_ascii_case("euler") {
            Ok(DiffuserSchedulerKind::Euler)
        }
        else {
            Err(format!("Unknown scheduler name `{s}`."))
        }
    }
}

impl<'de> Visitor<'de> for SchedulerKindDeserializer {
    type Value = DiffuserSchedulerKind;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Except one of `Euler`, `DDIM`")
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        unsafe { Ok(transmute::<u8, DiffuserSchedulerKind>(v as u8)) }
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        DiffuserSchedulerKind::from_str(v).map_err(|e| Error::custom(e))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(String::from_utf8_lossy(v).as_ref())
    }
}

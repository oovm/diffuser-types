use std::{error::Error, fmt::Formatter, mem::transmute};

use crate::kinds::DiffuserSchedulerKind;

struct SchedulerKindVisitor;

impl<'de> Deserialize<'de> for DiffuserSchedulerKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SchedulerKindVisitor)
    }
}

impl<'de> Visitor<'de> for SchedulerKindVisitor {
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
        if v.eq_ignore_ascii_case("ddim") {
            Ok(DiffuserSchedulerKind::DDIM)
        }
        else if v.eq_ignore_ascii_case("euler") {
            Ok(DiffuserSchedulerKind::Euler)
        }
        else {
            Err(E::custom("Unknown scheduler type"))
        }
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(String::from_utf8_lossy(v).as_ref())
    }
}

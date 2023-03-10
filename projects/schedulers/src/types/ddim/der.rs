use std::fmt::Formatter;

use serde::{
    __private::de::Content,
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::DDIMScheduler;

struct DDIMDeserialize<'i> {
    place: &'i mut DDIMScheduler,
}

impl<'de> Deserialize<'de> for DDIMScheduler {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut out = DDIMScheduler::default();
        DDIMScheduler::deserialize_in_place(deserializer, &mut out)?;
        Ok(out)
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(DDIMDeserialize { place })
    }
}

impl<'i, 'de> Visitor<'de> for DDIMDeserialize<'i> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Except scheduler name or Scheduler object.")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "init_noise_sigma" => {
                    self.place.init_noise_sigma = map.next_value()?;
                }
                _ => {
                    log::info!("Unknown field: {key}");
                    let _ = map.next_value::<Content>()?;
                }
            }
        }
        Ok(())
    }
}

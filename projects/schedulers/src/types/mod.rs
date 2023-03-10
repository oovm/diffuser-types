use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

pub use self::{ddim::DDIMScheduler, euler::EulerDiscreteScheduler};

mod ddim;
mod der;
mod euler;

#[derive(Clone, Serialize)]
#[serde(tag = "type")]
pub enum DiffuserScheduler {
    #[doc = include_str!("euler/Readme.md")]
    Euler(Box<EulerDiscreteScheduler>),
    #[doc = include_str!("ddim/Readme.md")]
    DDIM(Box<DDIMScheduler>),
}

impl Default for DiffuserScheduler {
    fn default() -> Self {
        Self::DDIM(Box::new(DDIMScheduler::default()))
    }
}

impl Debug for DiffuserScheduler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DiffuserScheduler::Euler(v) => Debug::fmt(v, f),
            DiffuserScheduler::DDIM(v) => Debug::fmt(v, f),
        }
    }
}

impl Display for DiffuserScheduler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DiffuserScheduler::Euler(v) => Debug::fmt(v, f),
            DiffuserScheduler::DDIM(v) => Debug::fmt(v, f),
        }
    }
}

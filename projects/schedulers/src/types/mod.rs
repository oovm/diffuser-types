use serde::{Deserialize, Serialize};

pub use self::{ddim::DDIMScheduler, euler::EulerScheduler};

mod ddim;
mod der;
mod euler;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
pub enum DiffuserScheduler {
    Euler(Box<EulerScheduler>),
    DDIM(Box<DDIMScheduler>),
}

impl Default for DiffuserScheduler {
    fn default() -> Self {
        Self::DDIM(Box::new(DDIMScheduler::default()))
    }
}

impl Default for DiffuserSchedulerKind {
    fn default() -> Self {
        Self::DDIM
    }
}

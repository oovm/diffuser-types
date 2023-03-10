use crate::DiffuserScheduler;
use serde::Serialize;

#[repr(u8)]
#[derive(Clone, Debug, Serialize)]
pub enum DiffuserSchedulerKind {
    Euler,
    DDIM,
}

mod der;

impl Default for DiffuserSchedulerKind {
    fn default() -> Self {
        Self::DDIM
    }
}

impl DiffuserSchedulerKind {
    pub fn as_scheduler(&self) -> DiffuserScheduler {
        match self {
            DiffuserSchedulerKind::Euler => DiffuserScheduler::Euler(Box::default()),
            DiffuserSchedulerKind::DDIM => DiffuserScheduler::DDIM(Box::default()),
        }
    }
}

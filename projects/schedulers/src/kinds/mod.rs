use serde::Serialize;

use crate::DiffuserScheduler;

#[repr(usize)]
#[derive(Clone, Debug, Serialize)]
pub enum DiffuserSchedulerKind {
    EulerDiscrete = 100,
    EulerAncestralDiscrete = 101,
    DDIM = 200,
    DDPM = 201,
    LMSDiscrete,
    /// DMP++
    DPMSolver,
    /// DMP++
    DPMSolverPlusPlus,
}

mod der;

impl Default for DiffuserSchedulerKind {
    fn default() -> Self {
        Self::DPMSolverPlusPlus
    }
}

impl DiffuserSchedulerKind {
    pub fn as_scheduler(&self) -> DiffuserScheduler {
        match self {
            DiffuserSchedulerKind::EulerDiscrete => DiffuserScheduler::Euler(Box::default()),
            DiffuserSchedulerKind::DDIM => DiffuserScheduler::DDIM(Box::default()),
            DiffuserSchedulerKind::LMSDiscrete => {
                unimplemented!()
            }
            DiffuserSchedulerKind::DPMSolver => {
                unimplemented!()
            }
            DiffuserSchedulerKind::DPMSolverPlusPlus => {
                unimplemented!()
            }
            DiffuserSchedulerKind::EulerAncestralDiscrete => {
                unimplemented!()
            }
            DiffuserSchedulerKind::DDPM => {
                unimplemented!()
            }
        }
    }
}

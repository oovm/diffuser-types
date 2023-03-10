#[repr(u8)]
#[derive(Clone, Debug, Serialize)]
pub enum DiffuserSchedulerKind {
    Euler,
    DDIM,
}

mod der;

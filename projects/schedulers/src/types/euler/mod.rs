use super::*;

#[doc = include_str!("Readme.md")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EulerDiscreteScheduler {
    pub alphas_cumprod: f32,
    pub sigmas: f32,
    pub init_noise_sigma: f32,
}

impl Default for EulerDiscreteScheduler {
    fn default() -> Self {
        Self { alphas_cumprod: 0.0, sigmas: 0.0, init_noise_sigma: 0.0 }
    }
}

impl From<EulerDiscreteScheduler> for DiffuserScheduler {
    fn from(value: EulerDiscreteScheduler) -> Self {
        DiffuserScheduler::Euler(Box::new(value))
    }
}

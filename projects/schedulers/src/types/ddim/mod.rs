use super::*;

#[doc = include_str!("Readme.md")]
#[derive(Clone, Debug, Serialize)]
pub struct DDIMScheduler {
    pub alphas_cumprod: f32,
    pub final_alpha_cumprod: f32,
    pub init_noise_sigma: f32,
}

impl Default for DDIMScheduler {
    fn default() -> Self {
        Self { alphas_cumprod: 0.0, final_alpha_cumprod: 0.0, init_noise_sigma: 0.0 }
    }
}

mod der;

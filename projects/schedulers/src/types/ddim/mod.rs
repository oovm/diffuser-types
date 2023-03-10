use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DDIMScheduler {}

impl Default for DDIMScheduler {
    fn default() -> Self {
        Self {}
    }
}

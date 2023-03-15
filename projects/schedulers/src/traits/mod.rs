pub trait Scheduler {
    /// If it's a linear scheduler
    const LINEAR: bool;
}

pub trait StableDiffusionScheduler {
    fn stable_diffusion_v1_optimized() -> Self;
}

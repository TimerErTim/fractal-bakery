#[derive(Clone)]
pub struct RenderingSettings {
    pub resolution: Resolution,
    pub sampling: MultiSampling,
}

#[derive(Copy, Clone)]
pub struct Resolution {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[derive(Clone)]
pub enum MultiSampling {
    NONE,
    X2,
    X4,
}
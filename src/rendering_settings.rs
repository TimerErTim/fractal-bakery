pub struct RenderingSettings {
    pub resolution: Resolution,
    pub sampling: MultiSampling,
}

pub struct Resolution {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

pub enum MultiSampling {
    NONE,
    X2,
    X4,
}
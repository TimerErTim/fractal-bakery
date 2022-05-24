pub struct RenderingSettings {
    pub resolution: Resolution,
    pub sampling: MultiSampling,
}

pub struct Resolution {
    width: u32,
    height: u32,
}

pub enum MultiSampling {
    NONE,
    X2,
    X4,
}
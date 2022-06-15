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

impl MultiSampling {
    pub fn samples_x(&self) -> u8 {
        match self {
            MultiSampling::NONE => 1,
            MultiSampling::X2 => 2,
            MultiSampling::X4 => 2
        }
    }

    pub fn samples_y(&self) -> u8 {
        match self {
            MultiSampling::NONE => 1,
            MultiSampling::X2 => 1,
            MultiSampling::X4 => 2
        }
    }
}
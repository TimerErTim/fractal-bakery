use crate::fractal::fractal::Configuration;

pub struct AnimationIterator<C: Configuration> {
    counter: u64,
    animation: Box<dyn FractalAnimation<C, IntoIter=AnimationIterator<C>, Item=Frame<C>>>,
}

impl<C: Configuration> AnimationIterator<C> {
    pub fn new(animation: impl FractalAnimation<C, IntoIter=AnimationIterator<C>, Item=Frame<C>> + 'static) -> Self {
        Self {
            counter: 0,
            animation: Box::new(animation),
        }
    }
}

unsafe impl<C: Configuration> Send for AnimationIterator<C> {
    // empty
}

pub struct Frame<C: Configuration> {
    pub number: u64,
    pub config: C,
}

impl<C: Configuration> Frame<C> {
    fn new(number: u64, config: C) -> Self {
        Self {
            number,
            config,
        }
    }
}

impl<C: Configuration> Iterator for AnimationIterator<C> {
    type Item = Frame<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        if let Some(config) = self.animation.get_configuration(self.counter) {
            Some(Frame::new(self.counter, config))
        } else {
            None
        }
    }
}

pub trait FractalAnimation<C: Configuration>: IntoIterator {
    fn get_configuration(&mut self, index: u64) -> Option<C>;
}
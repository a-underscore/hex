use glium::{
    texture::{MipmapsOption, Texture2d, Texture2dDataSource},
    uniforms::SamplerBehavior,
    Display,
};
use std::rc::Rc;

#[derive(Clone)]
pub struct Texture {
    pub buffer: Rc<Texture2d>,
    pub sampler_behaviour: SamplerBehavior,
}

impl Texture {
    pub fn new<'a, T>(
        display: &Display,
        source: T,
        mipmaps_option: MipmapsOption,
        sampler_behaviour: SamplerBehavior,
    ) -> anyhow::Result<Self>
    where
        T: Texture2dDataSource<'a>,
    {
        Ok(Self {
            buffer: Rc::new(Texture2d::with_mipmaps(display, source, mipmaps_option)?),
            sampler_behaviour,
        })
    }
}

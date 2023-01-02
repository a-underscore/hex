use glium::{
    texture::{MipmapsOption, RawImage2d, Texture2d},
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
    pub fn new(
        display: &Display,
        image: RawImage2d<u8>,
        sampler_behaviour: SamplerBehavior,
        mipmaps_option: MipmapsOption,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            buffer: Rc::new(Texture2d::with_mipmaps(display, image, mipmaps_option)?),
            sampler_behaviour,
        })
    }
}

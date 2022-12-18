use glium::{
    texture::{MipmapsOption, RawImage2d, Texture2d},
    uniforms::SamplerBehavior,
    Display,
};
use std::rc::Rc;

pub struct Texture {
    pub buffer: Texture2d,
    pub sampler_behaviour: SamplerBehavior,
}

impl Texture {
    pub fn new(
        display: &Rc<Display>,
        image: RawImage2d<u8>,
        sampler_behaviour: SamplerBehavior,
        mipmaps_option: MipmapsOption,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            buffer: Texture2d::with_mipmaps(display.as_ref(), image, mipmaps_option)?,
            sampler_behaviour,
        })
    }
}

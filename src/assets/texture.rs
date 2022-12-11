use crate::ecs::{self, Type};
use glium::{
    texture::{MipmapsOption, RawImage2d, Texture2d},
    uniforms::SamplerBehavior,
    Display,
};

pub struct Texture {
    pub buffer: Texture2d,
    pub sampler_behaviour: SamplerBehavior,
}

impl Texture {
    pub fn new(
        display: &Display,
        image: RawImage2d<u8>,
        sampler_behaviour: SamplerBehavior,
        mipmaps_option: MipmapsOption,
    ) -> anyhow::Result<Type<Self>> {
        Ok(ecs::new(Self {
            buffer: Texture2d::with_mipmaps(display, image, mipmaps_option)?,
            sampler_behaviour,
        }))
    }
}

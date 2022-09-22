use super::TextureBuffer;
use glium::{
    texture::{bindless::TextureHandle, MipmapsOption, RawImage2d, ResidentTexture, Texture2d},
    uniforms::SamplerBehavior,
    Display,
};
use std::{cell::RefCell, rc::Rc};

pub struct Texture {
    pub texture: ResidentTexture,
    pub sampler_behaviour: SamplerBehavior,
}

impl Texture {
    pub fn new(
        display: &Display,
        image: RawImage2d<u8>,
        sampler_behaviour: SamplerBehavior,
        mipmaps_option: MipmapsOption,
    ) -> anyhow::Result<Rc<RefCell<Self>>> {
        Ok(Rc::new(RefCell::new(Self {
            texture: Texture2d::with_mipmaps(display, image, mipmaps_option)?
                .resident()
                .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?,
            sampler_behaviour,
        })))
    }
}

impl TextureBuffer for Texture {
    fn handle(&mut self) -> anyhow::Result<TextureHandle> {
        let sampler_behaviour = self.sampler_behaviour;

        Ok(TextureHandle::new(&self.texture, &sampler_behaviour))
    }
}

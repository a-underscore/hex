use super::TextureBuffer;
use glium::{
    texture::{bindless::TextureHandle, MipmapsOption, RawImage2d, ResidentTexture, Texture2d},
    Display,
};
use std::{cell::RefCell, rc::Rc};

pub struct Texture {
    pub texture: ResidentTexture,
    pub mipmaps_option: MipmapsOption,
}

impl Texture {
    pub fn new(
        display: &Display,
        image: RawImage2d<u8>,
        mipmaps_option: MipmapsOption,
    ) -> anyhow::Result<Rc<RefCell<Self>>> {
        Ok(Rc::new(RefCell::new(Self {
            texture: Texture2d::with_mipmaps(display, image, mipmaps_option)?
                .resident()
                .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?,
            mipmaps_option,
        })))
    }
}

impl TextureBuffer for Texture {
    fn unit<'a>(&'a mut self) -> anyhow::Result<TextureHandle> {
        Ok(TextureHandle::new(&self.texture, &Default::default()))
    }
}

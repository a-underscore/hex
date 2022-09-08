use super::TextureBuffer;
use glium::{
    texture::{MipmapsOption, RawImage2d, SrgbTexture2d},
    Display,
};
use std::{cell::RefCell, rc::Rc};

pub struct Texture<'a> {
    pub image: RawImage2d<'a, u8>,
    pub mipmaps_option: MipmapsOption,
}

impl<'a> Texture<'a> {
    pub fn new(
        image: RawImage2d<'a, u8>,
        mipmaps_option: MipmapsOption,
    ) -> anyhow::Result<Rc<RefCell<Self>>> {
        Ok(Rc::new(RefCell::new(Self {
            image,
            mipmaps_option,
        })))
    }
}

impl<'a> TextureBuffer for Texture<'a> {
    fn bind(&mut self, display: &Display) -> anyhow::Result<SrgbTexture2d> {
        Ok(SrgbTexture2d::with_mipmaps(
            display,
            RawImage2d {
                data: self.image.data.clone(),
                ..self.image
            },
            self.mipmaps_option,
        )?)
    }
}

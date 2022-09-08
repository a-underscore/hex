use super::TextureBuffer;
use glium::{
    texture::{MipmapsOption, RawImage2d, SrgbTexture2d},
    Display,
};
use std::{cell::RefCell, rc::Rc};

pub struct Texture<'a, F>
where
    F: Fn() -> RawImage2d<'a, u8>,
{
    pub process: F,
    pub mipmaps_option: MipmapsOption,
}

impl<'a, F> Texture<'a, F>
where
    F: Fn() -> RawImage2d<'a, u8>,
{
    pub fn new(process: F, mipmaps_option: MipmapsOption) -> anyhow::Result<Rc<RefCell<Self>>> {
        Ok(Rc::new(RefCell::new(Self {
            process,
            mipmaps_option,
        })))
    }
}

impl<'a, F> TextureBuffer for Texture<'a, F>
where
    F: Fn() -> RawImage2d<'a, u8>,
{
    fn bind(&self, display: &Display) -> anyhow::Result<SrgbTexture2d> {
        Ok(SrgbTexture2d::with_mipmaps(
            display,
            (self.process)(),
            self.mipmaps_option,
        )?)
    }
}

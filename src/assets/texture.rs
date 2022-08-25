use glium::{
    texture::{MipmapsOption, RawImage2d, SrgbTexture2d},
    Display,
};
use std::{cell::RefCell, rc::Rc};

pub struct Texture {
    pub texture: SrgbTexture2d,
}

impl Texture {
    pub fn new(
        display: &Display,
        image: RawImage2d<u8>,
        mipmaps_option: MipmapsOption,
    ) -> anyhow::Result<Rc<RefCell<Self>>> {
        Ok(Rc::new(RefCell::new(Self {
            texture: SrgbTexture2d::with_mipmaps(display, image, mipmaps_option)?,
        })))
    }
}

use glium::{
    texture::{RawImage2d, SrgbTexture2d},
    Display,
};
use std::rc::Rc;

pub struct Texture {
    pub texture: SrgbTexture2d,
}

impl Texture {
    pub fn new(display: &Display, image: RawImage2d<u8>) -> anyhow::Result<Rc<Self>> {
        Ok(Rc::new(Self {
            texture: SrgbTexture2d::new(display, image)?,
        }))
    }
}

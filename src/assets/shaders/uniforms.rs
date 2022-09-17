use glium::{implement_uniform_block, texture::bindless::TextureHandle};

#[derive(Copy, Clone)]
pub struct Uniforms<'a> {
    pub image: TextureHandle<'a>,
}

impl<'a> Uniforms<'a> {
    pub fn new(image: TextureHandle<'a>) -> Self {
        Self { image }
    }
}

implement_uniform_block! {
    Uniforms<'a>, image
}

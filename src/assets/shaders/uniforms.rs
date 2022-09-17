use glium::{implement_uniform_block, texture::bindless::TextureHandle};

#[derive(Copy, Clone)]
pub struct Uniforms<'a> {
    pub image: TextureHandle<'a>,
}

implement_uniform_block! {
    Uniforms<'a>, image
}

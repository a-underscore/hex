use glium::{texture::SrgbTexture2d, Display};

pub trait TextureBuffer {
    fn bind(&mut self, display: &Display) -> anyhow::Result<SrgbTexture2d>;
}

use glium::{texture::SrgbTexture2d, Display};

pub trait TextureBuffer {
    fn bind(&self, display: &Display) -> anyhow::Result<SrgbTexture2d>;
}

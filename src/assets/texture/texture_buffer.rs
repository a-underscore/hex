use glium::texture::TextureHandle;

pub trait TextureBuffer {
    fn unit(&mut self) -> anyhow::Result<TextureHandle>;
}

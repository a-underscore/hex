use glium::texture::TextureHandle;

pub trait TextureBuffer {
    fn handle(&mut self) -> anyhow::Result<TextureHandle>;
}

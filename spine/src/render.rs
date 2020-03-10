use super::{result::Result, texture::Texture};

pub struct Vertex {
    pub position: [f32; 2],
    pub texture_coords: [f32; 2],
}

pub trait Renderer {
    type Texture: Texture;
    type Frame;

    fn prepare(&self) -> Self::Frame;

    fn render(
        &self,
        frame: &mut Self::Frame,
        vertices: &[Vertex],
        texture: &Self::Texture,
    ) -> Result<()>;

    fn finish(&self, frame: Self::Frame) -> Result<()>;
}

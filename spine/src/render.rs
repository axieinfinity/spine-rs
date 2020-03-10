use super::result::Result;

use crate::skeleton::Skeleton;
use image::DynamicImage;

#[allow(non_snake_case)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub a_position: [f32; 2],
    pub a_texCoords: [f32; 2],
}

pub trait Renderer {
    type Frame;

    fn prepare_frame(&self) -> Self::Frame;

    fn render_in_frame(
        &self,
        frame: &mut Self::Frame,
        vertices: &[Vertex],
        texture: &DynamicImage,
    ) -> Result<()>;

    fn finish_frame(&self, frame: Self::Frame) -> Result<()>;

    fn render(&self, _skeleton: &Skeleton) -> Result<()> {
        unimplemented!()
    }
}

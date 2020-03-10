use std::error::Error;

use glium::Surface;
use image::{DynamicImage, GenericImageView};
use spine::Renderer;

#[repr(transparent)]
struct Texture(DynamicImage);

impl spine::Texture for Texture {
    #[inline]
    fn width(&self) -> u32 {
        self.0.dimensions().0
    }

    #[inline]
    fn height(&self) -> u32 {
        self.0.dimensions().0
    }
}

fn read_texture_file(path: &str) -> spine::Result<Texture> {
    image::open(path)
        .map(Texture)
        .map_err(|error| spine::Error::Other(Box::new(error) as Box<dyn Error>))
}

spine::impl_spine!(DynamicImage, read_texture_file);

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    texture_coords: [f32; 2],
}

glium::implement_vertex!(Vertex, position, texture_coords);

struct Display(glium::Display);

impl Renderer for Display {
    type Texture = Texture;
    type Frame = glium::Frame;

    #[inline]
    fn prepare(&self) -> Self::Frame {
        self.0.draw()
    }

    fn render(
        &self,
        frame: &mut Self::Frame,
        vertices: &[spine::Vertex],
        texture: &Self::Texture,
    ) -> spine::Result<()> {
        let vertices = unsafe {
            std::slice::from_raw_parts(vertices.as_ptr() as *const Vertex, vertices.len())
        };

        let vertex_buffer =
            glium::VertexBuffer::new(&self.0, vertices).map_err(spine::Error::render)?;

        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            )
            .map_err(spine::Error::render)
    }

    fn finish(&self, frame: &mut Self::Frame) -> spine::Result<()> {
        frame.finish().map_err(spine::Error::render)
    }
}

use glium::{
    implement_vertex,
    index::{NoIndices, PrimitiveType},
    texture::SrgbTexture2d,
    uniform, Blend, Display, DrawParameters, Frame, Program, Surface, VertexBuffer,
};
use image::{DynamicImage, GenericImageView};

use crate::{
    error::Error,
    render::{Renderer, Vertex},
    result::Result,
};
use glium::texture::RawImage2d;

implement_vertex!(Vertex, a_position, a_texCoords);

pub struct GliumRenderer<'a> {
    display: &'a Display,
    program: Program,
    draw_parameters: DrawParameters<'a>,
}

impl<'a> GliumRenderer<'a> {
    pub fn new(display: &'a Display) -> Result<Self> {
        let vertex_shader = include_str!("../shader/spine.vert");
        let fragment_shader = include_str!("../shader/spine.frag");

        let program = Program::from_source(display, vertex_shader, fragment_shader, None)
            .map_err(Error::render)?;

        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        Ok(Self {
            display,
            program,
            draw_parameters,
        })
    }
}

impl<'a> Renderer for GliumRenderer<'a> {
    type Frame = Frame;

    #[inline]
    fn prepare_frame(&self) -> Self::Frame {
        self.display.draw()
    }

    fn render_in_frame(
        &self,
        frame: &mut Self::Frame,
        vertices: &[Vertex],
        texture: &DynamicImage,
    ) -> Result<()> {
        let vertex_buffer = VertexBuffer::new(self.display, vertices).map_err(Error::render)?;
        let index_buffer = NoIndices(PrimitiveType::TrianglesList);

        let perspective = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

        let image = RawImage2d::from_raw_rgba_reversed(
            &texture.to_bytes(),
            (texture.width(), texture.height()),
        );

        let texture = SrgbTexture2d::new(self.display, image).map_err(Error::render)?;

        let uniforms = uniform! {
            perspective: perspective,
            texture: &texture,
        };

        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                &self.program,
                &uniforms,
                &self.draw_parameters,
            )
            .map_err(Error::render)
    }

    #[inline]
    fn finish_frame(&self, frame: Self::Frame) -> Result<()> {
        frame.finish().map_err(Error::render)
    }
}

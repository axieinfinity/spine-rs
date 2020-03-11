use std::collections::HashMap;

use glium::{
    implement_vertex,
    index::{NoIndices, PrimitiveType},
    texture::{RawImage2d, SrgbTexture2d},
    uniform, Blend, Display, DrawParameters, Frame, Program, Surface, VertexBuffer,
};
use image::GenericImageView;

use super::{
    atlas::{Atlas, AtlasPage},
    error::Error,
    render::{Renderer, Vertex},
    result::Result,
};

implement_vertex!(Vertex, in_position, in_texture_coords);

pub struct GliumRenderer<'a> {
    display: Display,
    program: Program,
    draw_parameters: DrawParameters<'a>,
    textures: HashMap<usize, SrgbTexture2d>,
}

impl<'a> GliumRenderer<'a> {
    pub fn new(display: Display) -> Result<Self> {
        let vertex_shader = include_str!("./shaders/spine.vert");
        let fragment_shader = include_str!("./shaders/spine.frag");

        let program = Program::from_source(&display, vertex_shader, fragment_shader, None)
            .map_err(Error::render)?;

        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        Ok(Self {
            display,
            program,
            draw_parameters,
            textures: HashMap::new(),
        })
    }

    #[inline]
    pub fn display(&self) -> &Display {
        &self.display
    }

    #[inline]
    pub fn build_textures(&mut self, atlas: &Atlas) -> Result<()> {
        let Self {
            display, textures, ..
        } = self;

        atlas.build_textures(textures, |texture| {
            let image = RawImage2d::from_raw_rgba_reversed(
                &texture.to_bytes(),
                (texture.width(), texture.height()),
            );

            SrgbTexture2d::new(display, image).map_err(Error::render)
        })
    }
}

impl<'a> Renderer for GliumRenderer<'a> {
    type Frame = Frame;

    fn render_in_frame(
        &self,
        vertices: &[Vertex],
        page: &AtlasPage,
        frame: &mut Self::Frame,
    ) -> Result<()> {
        let vertex_buffer = VertexBuffer::new(&self.display, vertices).map_err(Error::render)?;
        let index_buffer = NoIndices(PrimitiveType::TrianglesList);

        let (width, height) = frame.get_dimensions();
        let (width, height) = (width as f32, height as f32);

        let perspective = [
            [1.0 / width, 0.0, 0.0],
            [0.0, 1.0 / height, 0.0],
            [0.0, 0.0, 1.0],
        ];

        let texture = self.textures.get(&page.id()).unwrap();

        let uniforms = uniform! {
            u_perspective: perspective,
            u_texture: texture,
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
}

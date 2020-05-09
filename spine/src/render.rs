use std::{path::Path, rc::Rc};

use image::DynamicImage;

use super::{atlas::Atlas, error::Error, geometry::Vertex, result::Result, skeleton::Skeleton};

pub trait Renderer: Sized {
    type Texture;
    type Frame;

    fn build_texture(&self, texture: &DynamicImage) -> Result<Self::Texture>;
    fn add_texture(&mut self, id: usize, texture: Self::Texture);
    fn get_texture(&self, id: &usize) -> Option<&Self::Texture>;

    fn new_atlas(&mut self, path: impl AsRef<Path>) -> Result<Rc<Atlas>> {
        let atlas = Atlas::new(path)?;
        atlas.build_textures(self)?;
        Ok(atlas)
    }

    fn render_mesh(
        &self,
        vertices: &[Vertex],
        texture: &Self::Texture,
        frame: &mut Self::Frame,
    ) -> Result<()>;

    fn render(&self, skeleton: &mut Skeleton, frame: &mut Self::Frame) -> Result<()> {
        skeleton.for_each_mesh(|vertices, page| {
            let texture = self
                .get_texture(&page.id())
                .ok_or(Error::invalid_data(format!(
                    "texture of page named \"{}\" has not been initialized",
                    page.name()?,
                )))?;

            self.render_mesh(&vertices, texture, frame)
        })
    }
}

use image::DynamicImage;
use std::rc::Rc;

use super::{
    atlas::{Atlas, AtlasPage},
    error::Error,
    result::Result,
    skeleton::{Attachment, Skeleton},
};

const MAX_VERTICES_PER_ATTACHMENT: usize = 2048;

const OFFSET: usize = 0;
const STRIDE: usize = 2;

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub in_position: [f32; 2],
    pub in_texture_coords: [f32; 2],
}

pub trait Renderer: Sized {
    type Texture;
    type Frame;

    fn build_texture(&self, texture: &DynamicImage) -> Result<Self::Texture>;
    fn add_texture(&mut self, id: usize, texture: Self::Texture);
    fn get_texture(&self, id: &usize) -> Option<&Self::Texture>;

    fn new_atlas(&mut self, path: &str) -> Result<Rc<Atlas>> {
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
        let mut render_mesh = |vertices: &mut Vec<Vertex>, page: &AtlasPage| -> Result<()> {
            let texture = self
                .get_texture(&page.id())
                .ok_or(Error::invalid_data(format!(
                    "texture of page named \"{}\" has not been initialized",
                    page.name()?,
                )))?;

            self.render_mesh(&vertices, texture, frame)?;
            vertices.clear();

            Ok(())
        };

        let mut last_page: Option<AtlasPage> = None;
        let mut vertices = Vec::new();

        let mut world_vertices = vec![0.0; MAX_VERTICES_PER_ATTACHMENT];

        for slot in skeleton.slots_ordered() {
            let mut attachment = match slot.attachment() {
                Some(attachment) => attachment,
                None => continue,
            };

            let (indices, uvs, page) = match attachment {
                Attachment::Mesh(ref mut mesh) => {
                    let page = mesh.region().page();
                    let world_vertices_len = mesh.world_vertices_len();

                    mesh.compute_world_vertices(
                        slot,
                        0,
                        world_vertices_len,
                        &mut world_vertices,
                        OFFSET,
                        STRIDE,
                    );

                    (mesh.triangles(), mesh.uvs(), page)
                }

                Attachment::Region(ref region) => {
                    let page = region.region().page();

                    region.compute_world_vertices(
                        &slot.bone(),
                        &mut world_vertices,
                        OFFSET,
                        STRIDE,
                    );

                    (&QUAD_INDICES[..], region.uvs(), page)
                }

                _ => continue,
            };

            if let Some(ref last_page) = last_page {
                if page.id() != last_page.id() {
                    render_mesh(&mut vertices, last_page)?;
                }
            }

            for index in indices {
                let index = (*index as usize) << 1;

                vertices.push(Vertex {
                    in_position: [world_vertices[index], world_vertices[index + 1]],
                    in_texture_coords: [uvs[index], -uvs[index + 1]],
                })
            }

            last_page = Some(page);
        }

        if let Some(ref last_page) = last_page {
            render_mesh(&mut vertices, last_page)?;
        }

        Ok(())
    }
}

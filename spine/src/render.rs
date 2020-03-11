use image::DynamicImage;

use super::{attachment::Attachment, result::Result, skeleton::Skeleton};

const MAX_VERTICES_PER_ATTACHMENT: usize = 2048;

const OFFSET: usize = 0;
const STRIDE: usize = 2;

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub a_position: [f32; 2],
    pub a_texCoords: [f32; 2],
}

pub trait Renderer {
    type Frame;

    fn render_in_frame(
        &self,
        vertices: &[Vertex],
        texture: &DynamicImage,
        frame: &mut Self::Frame,
    ) -> Result<()>;

    fn render(&self, skeleton: &mut Skeleton, frame: &mut Self::Frame) -> Result<()> {
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

            let mut vertices = Vec::with_capacity(indices.len());

            for index in indices {
                let index = (*index as usize) << 1;

                vertices.push(Vertex {
                    a_position: [world_vertices[index], world_vertices[index + 1]],
                    a_texCoords: [uvs[index], -uvs[index + 1]],
                })
            }

            let texture = page.texture();

            self.render_in_frame(&vertices, texture, frame)?;
        }

        Ok(())
    }
}

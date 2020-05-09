use std::{ffi::CString, marker::PhantomData, ptr::NonNull, rc::Rc, slice};

use spine_sys::{
    spSkeleton, spSkeleton_create, spSkeleton_dispose, spSkeleton_findSlot,
    spSkeleton_setAttachment, spSkeleton_updateWorldTransform,
};

use crate::{
    atlas::AtlasPage,
    geometry::{Bounds, Vertex},
    result::Result,
};

use super::{attachment::Attachment, data::SkeletonData, slot::Slot};

pub const MAX_VERTICES_PER_ATTACHMENT: usize = 2048;

pub const OFFSET: usize = 0;
pub const STRIDE: usize = 2;

pub const QUAD_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

pub struct Skeleton {
    pub(crate) pointer: NonNull<spSkeleton>,
    pub(crate) _skeleton_data: Rc<SkeletonData>,
}

impl Skeleton {
    pub fn new(skeleton_data: &Rc<SkeletonData>) -> Self {
        let pointer = unsafe { spSkeleton_create(skeleton_data.pointer.as_ptr()) };

        Skeleton {
            pointer: NonNull::new(pointer).unwrap(),
            _skeleton_data: skeleton_data.clone(),
        }
    }

    pub fn get_bounds(&mut self) -> Bounds {
        let mut bounds = Bounds::dummy();

        self.for_each_mesh(|vertices, _| {
            for vertex in vertices {
                bounds.cover(vertex.in_position[0], vertex.in_position[1]);
            }

            Ok(())
        })
        .unwrap();

        bounds
    }

    pub fn set_x(&mut self, x: f32) -> &mut Self {
        unsafe { self.pointer.as_mut().x = x }
        self
    }

    pub fn set_y(&mut self, y: f32) -> &mut Self {
        unsafe { self.pointer.as_mut().y = y }
        self
    }
}

macro_rules! impl_slots {
    ($method:ident, $from_raw_parts_mut:ident, $slots:ident $(,$mut:tt)*) => {
        pub fn $method(&$($mut)* self) -> &$($mut)* [Slot] {
            unsafe {
                let skeleton = self.pointer.as_ref();

                slice::$from_raw_parts_mut(
                    skeleton.$slots as *mut Slot,
                    skeleton.slotsCount as usize,
                )
            }
        }
    };
}

impl Skeleton {
    impl_slots!(slots, from_raw_parts, slots);
    // impl_slots!(slots_mut, from_raw_parts_mut, slots, mut);
    impl_slots!(slots_ordered, from_raw_parts, drawOrder);
    // impl_slots!(slots_ordered_mut, from_raw_parts_mut, drawOrder, mut);

    pub fn find_slot(&self, name: &str) -> Option<Slot> {
        let name = CString::new(name).ok()?;
        let pointer = unsafe { spSkeleton_findSlot(self.pointer.as_ptr(), name.as_ptr()) };
        let pointer = NonNull::new(pointer)?;
        Some(Slot(pointer, PhantomData))
    }

    pub fn set_attachment(
        &mut self,
        slot_name: &str,
        attachment_name: Option<&str>,
    ) -> Result<&mut Self> {
        let slot_name = CString::new(slot_name)?;

        let attachment_name = match attachment_name {
            Some(name) => CString::new(name)?.as_ptr(),
            None => std::ptr::null(),
        };

        unsafe {
            spSkeleton_setAttachment(self.pointer.as_ptr(), slot_name.as_ptr(), attachment_name);
        }

        Ok(self)
    }

    pub fn update_world_transform(&mut self) {
        unsafe { spSkeleton_updateWorldTransform(self.pointer.as_ptr()) }
    }

    pub fn for_each_mesh(
        &mut self,
        mut apply: impl FnMut(&[Vertex], &AtlasPage) -> Result<()>,
    ) -> Result<()> {
        let mut last_page: Option<AtlasPage> = None;
        let mut vertices = Vec::new();

        let mut world_vertices = vec![0.0; MAX_VERTICES_PER_ATTACHMENT];

        for slot in self.slots_ordered() {
            let attachment = match slot.attachment() {
                Some(attachment) => attachment,
                None => continue,
            };

            let (indices, uvs, page) = match &attachment {
                Attachment::Mesh(mesh) => {
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

                Attachment::Region(region) => {
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

            if let Some(last_page) = &last_page {
                if page.id() != last_page.id() {
                    apply(&vertices, last_page)?;
                    vertices.clear();
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

        if let Some(last_page) = &last_page {
            apply(&vertices, last_page)?;
            vertices.clear();
        }

        Ok(())
    }
}

impl Drop for Skeleton {
    fn drop(&mut self) {
        unsafe { spSkeleton_dispose(self.pointer.as_ptr()) }
    }
}

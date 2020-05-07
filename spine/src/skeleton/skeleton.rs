use std::{f32, ffi::CString, marker::PhantomData, ptr::NonNull, rc::Rc, slice};

use spine_sys::{
    spSkeleton, spSkeleton_create, spSkeleton_dispose, spSkeleton_findSlot,
    spSkeleton_updateWorldTransform, spSlot,
};

use super::{data::SkeletonData, slot::Slot, Attachment};

const MAX_VERTICES_PER_ATTACHMENT: usize = 2048;

const OFFSET: usize = 0;
const STRIDE: usize = 2;

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

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

    pub fn update_world_transform(&mut self) {
        unsafe { spSkeleton_updateWorldTransform(self.pointer.as_ptr()) }
    }

    pub fn find_slot(&mut self, name: &str) -> Option<Slot> {
        let name = CString::new(name).ok()?;
        let pointer =
            unsafe { spSkeleton_findSlot(self.pointer.as_ptr(), name.as_ptr()) as *mut spSlot };
        let pointer = NonNull::new(pointer)?;
        let slot = Slot(pointer, PhantomData);
        Some(slot)
    }

    pub fn get_bounds(&self) -> (f32, f32, f32, f32) {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        let mut world_vertices = vec![0.0; MAX_VERTICES_PER_ATTACHMENT];
        for slot in self.slots_ordered() {
            let mut attachment = match slot.attachment() {
                Some(attachment) => attachment,
                _ => continue,
            };

            let indices = match attachment {
                Attachment::Mesh(ref mut mesh) => {
                    let world_vertices_len = mesh.world_vertices_len();
                    mesh.compute_world_vertices(
                        slot,
                        0,
                        world_vertices_len,
                        &mut world_vertices,
                        OFFSET,
                        STRIDE,
                    );
                    mesh.triangles()
                }

                Attachment::Region(ref region) => {
                    region.compute_world_vertices(
                        &slot.bone(),
                        &mut world_vertices,
                        OFFSET,
                        STRIDE,
                    );
                    &QUAD_INDICES[..]
                }

                _ => continue,
            };

            for index in indices {
                let index = (*index as usize) << 1;
                let (x, y) = (world_vertices[index], world_vertices[index + 1]);
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_x.max(y);
            }
        }

        (min_x, min_y, max_x, max_y)
    }
}

impl Drop for Skeleton {
    fn drop(&mut self) {
        unsafe { spSkeleton_dispose(self.pointer.as_ptr()) }
    }
}

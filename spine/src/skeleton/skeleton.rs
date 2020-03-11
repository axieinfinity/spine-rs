use std::{ptr::NonNull, rc::Rc, slice};

use spine_sys::{
    spSkeleton, spSkeleton_create, spSkeleton_dispose, spSkeleton_updateWorldTransform,
};

use crate::slot::Slot;

use super::data::SkeletonData;

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
}

impl Drop for Skeleton {
    fn drop(&mut self) {
        unsafe { spSkeleton_dispose(self.pointer.as_ptr()) }
    }
}

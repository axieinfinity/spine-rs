use std::{marker::PhantomData, ptr::NonNull, slice};

use spine_sys::{
    spSkeleton, spSkeleton_create, spSkeleton_dispose, spSkeleton_updateWorldTransform,
};

use crate::slot::Slot;

use super::data::SkeletonData;

#[repr(transparent)]
pub struct Skeleton<'skel_data>(
    pub(crate) NonNull<spSkeleton>,
    pub(crate) PhantomData<&'skel_data ()>,
);

impl<'a> Skeleton<'a> {
    pub fn new(data: &'a SkeletonData) -> Self {
        let pointer = unsafe { spSkeleton_create(data.0.as_ptr()) };
        let pointer = NonNull::new(pointer).unwrap();
        Skeleton(pointer, PhantomData)
    }
}

macro_rules! impl_slots {
    ($method:ident, $from_raw_parts_mut:ident, $slots:ident $(,$mut:tt)*) => {
        pub fn $method(&$($mut)* self) -> &$($mut)* [Slot] {
            unsafe {
                let skeleton = self.0.as_ref();

                slice::$from_raw_parts_mut(
                    skeleton.$slots as *mut Slot,
                    skeleton.slotsCount as usize,
                )
            }
        }
    };
}

impl<'a> Skeleton<'a> {
    impl_slots!(slots, from_raw_parts, slots);
    // impl_slots!(slots_mut, from_raw_parts_mut, slots, mut);
    impl_slots!(slots_ordered, from_raw_parts, drawOrder);
    // impl_slots!(slots_ordered_mut, from_raw_parts_mut, drawOrder, mut);

    pub fn update_world_transform(&mut self) {
        unsafe { spSkeleton_updateWorldTransform(self.0.as_ptr()) }
    }
}

impl<'a> Drop for Skeleton<'a> {
    fn drop(&mut self) {
        unsafe { spSkeleton_dispose(self.0.as_ptr()) }
    }
}

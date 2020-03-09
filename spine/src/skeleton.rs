use std::{marker::PhantomData, ptr::NonNull, slice};

use spine_sys::{spSkeleton, spSkeleton_create, spSkeleton_dispose};

use super::{error::Error, result::Result, skeleton_data::SkeletonData, slot::Slot};

#[repr(transparent)]
pub struct Skeleton<'skel_data>(
    pub(crate) NonNull<spSkeleton>,
    pub(crate) PhantomData<&'skel_data ()>,
);

impl<'a> Skeleton<'a> {
    pub fn new(data: &'a SkeletonData) -> Result<Self> {
        let pointer = unsafe { spSkeleton_create(data.0.as_ptr()) };
        let pointer = NonNull::new(pointer).ok_or(Error::NullPointer)?;
        Ok(Skeleton(pointer, PhantomData))
    }
}

macro_rules! impl_slots {
    ($method: ident, $c_slots:ident) => {
        pub fn $method(&self) -> &[Slot] {
            unsafe {
                let skeleton = self.0.as_ref();

                slice::from_raw_parts(
                    skeleton.$c_slots as *const Slot,
                    skeleton.slotsCount as usize,
                )
            }
        }
    };
}

impl<'a> Skeleton<'a> {
    impl_slots!(slots, slots);
    impl_slots!(slots_ordered, drawOrder);
}

impl<'a> Drop for Skeleton<'a> {
    fn drop(&mut self) {
        unsafe { spSkeleton_dispose(self.0.as_ptr()) }
    }
}
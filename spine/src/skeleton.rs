use std::marker::PhantomData;
use std::ptr::NonNull;

use spine_sys::{spSkeleton, spSkeleton_create, spSkeleton_dispose};

use super::error::Error;
use super::result::Result;
use super::skeleton_data::SkeletonData;

#[repr(transparent)]
pub struct Skeleton<'a>(
    pub(crate) NonNull<spSkeleton>,
    pub(crate) PhantomData<&'a ()>,
);

impl<'a> Skeleton<'a> {
    pub fn new(skeleton_data: &'a SkeletonData) -> Result<Self> {
        let pointer = unsafe { spSkeleton_create(skeleton_data.0.as_ptr()) };
        let pointer = NonNull::new(pointer).ok_or(Error::NullPointer)?;
        Ok(Skeleton(pointer, PhantomData))
    }
}

impl<'a> Drop for Skeleton<'a> {
    fn drop(&mut self) {
        unsafe { spSkeleton_dispose(self.0.as_ptr()) }
    }
}

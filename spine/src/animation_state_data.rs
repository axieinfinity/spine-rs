use std::marker::PhantomData;
use std::ptr::NonNull;

use spine_sys::{spAnimationStateData, spAnimationStateData_create, spAnimationStateData_dispose};

use super::error::Error;
use super::result::Result;
use super::skeleton_data::SkeletonData;

#[repr(transparent)]
pub struct AnimationStateData<'a>(
    pub(crate) NonNull<spAnimationStateData>,
    pub(crate) PhantomData<&'a ()>,
);

impl<'a> AnimationStateData<'a> {
    pub fn new(skeleton_data: &'a SkeletonData) -> Result<Self> {
        let pointer = unsafe { spAnimationStateData_create(skeleton_data.0.as_ptr()) };
        let pointer = NonNull::new(pointer).ok_or(Error::NullPointer)?;
        Ok(AnimationStateData(pointer, PhantomData))
    }
}

impl<'a> Drop for AnimationStateData<'a> {
    fn drop(&mut self) {
        unsafe { spAnimationStateData_dispose(self.0.as_ptr()) }
    }
}

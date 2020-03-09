use std::{marker::PhantomData, ptr::NonNull};

use spine_sys::{spAnimationStateData, spAnimationStateData_create, spAnimationStateData_dispose};

use super::skeleton_data::SkeletonData;

#[repr(transparent)]
pub struct AnimationStateData<'skel_data>(
    pub(crate) NonNull<spAnimationStateData>,
    pub(crate) PhantomData<&'skel_data ()>,
);

impl<'a> AnimationStateData<'a> {
    pub fn new(skeleton_data: &'a SkeletonData) -> Self {
        let pointer = unsafe { spAnimationStateData_create(skeleton_data.0.as_ptr()) };
        let pointer = NonNull::new(pointer).unwrap();
        AnimationStateData(pointer, PhantomData)
    }
}

impl<'a> Drop for AnimationStateData<'a> {
    fn drop(&mut self) {
        unsafe { spAnimationStateData_dispose(self.0.as_ptr()) }
    }
}

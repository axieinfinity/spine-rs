use std::{marker::PhantomData, ptr::NonNull};

use spine_sys::{spAnimationState, spAnimationState_create, spAnimationState_dispose};

use super::{animation_state_data::AnimationStateData, error::Error, result::Result};

#[repr(transparent)]
pub struct AnimationState<'anim_state_data>(
    pub(crate) NonNull<spAnimationState>,
    pub(crate) PhantomData<&'anim_state_data ()>,
);

impl<'a> AnimationState<'a> {
    pub fn new(animation_state_data: &'a AnimationStateData) -> Result<Self> {
        let pointer = unsafe { spAnimationState_create(animation_state_data.0.as_ptr()) };
        let pointer = NonNull::new(pointer).ok_or(Error::NullPointer)?;
        Ok(AnimationState(pointer, PhantomData))
    }
}

impl<'a> Drop for AnimationState<'a> {
    fn drop(&mut self) {
        unsafe { spAnimationState_dispose(self.0.as_ptr()) }
    }
}

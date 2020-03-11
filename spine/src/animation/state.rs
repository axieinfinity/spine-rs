use std::{ffi::CString, marker::PhantomData, os::raw::c_int, ptr::NonNull};

use spine_sys::{
    spAnimationState, spAnimationState_apply, spAnimationState_create, spAnimationState_dispose,
    spAnimationState_setAnimationByName, spAnimationState_update,
};

use crate::{result::Result, skeleton::Skeleton};

use super::state_data::AnimationStateData;

#[repr(transparent)]
pub struct AnimationState<'anim_state_data>(
    pub(crate) NonNull<spAnimationState>,
    pub(crate) PhantomData<&'anim_state_data ()>,
);

impl<'a> AnimationState<'a> {
    pub fn new(animation_state_data: &'a AnimationStateData) -> Self {
        let pointer = unsafe { spAnimationState_create(animation_state_data.0.as_ptr()) };
        let pointer = NonNull::new(pointer).unwrap();
        AnimationState(pointer, PhantomData)
    }

    pub fn set_animation_by_name(
        &mut self,
        track_index: usize,
        animation_name: &str,
        r#loop: bool,
    ) -> Result<()> {
        let animation_name = CString::new(animation_name)?;

        let _track_entry = unsafe {
            spAnimationState_setAnimationByName(
                self.0.as_ptr(),
                track_index as c_int,
                animation_name.as_ptr(),
                r#loop as c_int,
            )
        };

        Ok(())
    }

    pub fn update(&mut self, delta: f32) {
        unsafe { spAnimationState_update(self.0.as_ptr(), delta) }
    }

    pub fn apply(&mut self, skeleton: &mut Skeleton) {
        let _result = unsafe { spAnimationState_apply(self.0.as_ptr(), skeleton.0.as_ptr()) };
    }
}

impl<'a> Drop for AnimationState<'a> {
    fn drop(&mut self) {
        unsafe { spAnimationState_dispose(self.0.as_ptr()) }
    }
}

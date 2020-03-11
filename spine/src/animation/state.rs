use std::{ffi::CString, os::raw::c_int, ptr::NonNull, rc::Rc};

use spine_sys::{
    spAnimationState, spAnimationState_apply, spAnimationState_create, spAnimationState_dispose,
    spAnimationState_setAnimationByName, spAnimationState_update,
};

use crate::{result::Result, skeleton::Skeleton};

use super::state_data::AnimationStateData;

pub struct AnimationState {
    pub(crate) pointer: NonNull<spAnimationState>,
    pub(crate) _animation_state_data: Rc<AnimationStateData>,
}

impl AnimationState {
    pub fn new(animation_state_data: &Rc<AnimationStateData>) -> Self {
        let pointer = unsafe { spAnimationState_create(animation_state_data.pointer.as_ptr()) };

        AnimationState {
            pointer: NonNull::new(pointer).unwrap(),
            _animation_state_data: animation_state_data.clone(),
        }
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
                self.pointer.as_ptr(),
                track_index as c_int,
                animation_name.as_ptr(),
                r#loop as c_int,
            )
        };

        Ok(())
    }

    pub fn update(&mut self, delta: f32) {
        unsafe { spAnimationState_update(self.pointer.as_ptr(), delta) }
    }

    pub fn apply(&mut self, skeleton: &mut Skeleton) {
        let _result =
            unsafe { spAnimationState_apply(self.pointer.as_ptr(), skeleton.pointer.as_ptr()) };
    }
}

impl Drop for AnimationState {
    fn drop(&mut self) {
        unsafe { spAnimationState_dispose(self.pointer.as_ptr()) }
    }
}

use std::{ptr::NonNull, rc::Rc};

use spine_sys::{spAnimationStateData, spAnimationStateData_create, spAnimationStateData_dispose};

use crate::skeleton::SkeletonData;

pub struct AnimationStateData {
    pub(crate) pointer: NonNull<spAnimationStateData>,
    pub(crate) _skeleton_data: Rc<SkeletonData>,
}

impl AnimationStateData {
    pub fn new(skeleton_data: &Rc<SkeletonData>) -> Rc<Self> {
        let pointer = unsafe { spAnimationStateData_create(skeleton_data.pointer.as_ptr()) };

        Rc::new(AnimationStateData {
            pointer: NonNull::new(pointer).unwrap(),
            _skeleton_data: skeleton_data.clone(),
        })
    }
}

impl Drop for AnimationStateData {
    fn drop(&mut self) {
        unsafe { spAnimationStateData_dispose(self.pointer.as_ptr()) }
    }
}

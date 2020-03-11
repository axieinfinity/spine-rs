use std::{ptr::NonNull, rc::Rc};

use spine_sys::{spSkeletonJson, spSkeletonJson_create, spSkeletonJson_dispose};

use crate::atlas::Atlas;

pub struct SkeletonJson {
    pub(crate) pointer: NonNull<spSkeletonJson>,
    pub(crate) _atlas: Rc<Atlas>,
}

impl SkeletonJson {
    pub fn new(atlas: &Rc<Atlas>) -> Self {
        let pointer = unsafe { spSkeletonJson_create(atlas.pointer.as_ptr()) };

        Self {
            pointer: NonNull::new(pointer).unwrap(),
            _atlas: atlas.clone(),
        }
    }

    pub fn set_scale(&mut self, scale: f32) -> &mut Self {
        unsafe { self.pointer.as_mut().scale = scale }
        self
    }
}

impl Drop for SkeletonJson {
    fn drop(&mut self) {
        unsafe { spSkeletonJson_dispose(self.pointer.as_ptr()) }
    }
}

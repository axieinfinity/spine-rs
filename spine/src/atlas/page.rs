use std::{marker::PhantomData, ptr::NonNull};

use image::DynamicImage;
use spine_sys::spAtlasPage;

#[repr(transparent)]
pub struct AtlasPage<'atlas> {
    pub(crate) pointer: NonNull<spAtlasPage>,
    pub(crate) _marker: PhantomData<&'atlas ()>,
}

impl<'a> AtlasPage<'a> {
    pub fn texture(&self) -> &DynamicImage {
        unsafe {
            let pointer = self.pointer.as_ref().rendererObject as *mut DynamicImage;
            pointer.as_ref().unwrap()
        }
    }
}

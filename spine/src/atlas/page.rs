use std::{marker::PhantomData, ptr::NonNull};

use image::DynamicImage;
use spine_sys::spAtlasPage;

#[repr(transparent)]
pub struct AtlasPage<'atlas>(
    pub(crate) NonNull<spAtlasPage>,
    pub(crate) PhantomData<&'atlas ()>,
);

impl<'a> AtlasPage<'a> {
    pub fn texture(&self) -> &DynamicImage {
        unsafe {
            let pointer = self.0.as_ref().rendererObject as *mut DynamicImage;
            pointer.as_ref().unwrap()
        }
    }
}

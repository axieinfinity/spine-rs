use std::{ffi::CStr, marker::PhantomData, ptr::NonNull};

use image::DynamicImage;
use spine_sys::spAtlasPage;

use crate::{error::Error, result::Result};

#[repr(transparent)]
pub struct AtlasPage<'atlas> {
    pub(crate) pointer: NonNull<spAtlasPage>,
    pub(crate) _marker: PhantomData<&'atlas ()>,
}

impl<'a> AtlasPage<'a> {
    #[inline]
    pub fn id(&self) -> usize {
        self.pointer.as_ptr() as usize
    }

    #[inline]
    pub fn name(&self) -> Result<&str> {
        let name = unsafe { CStr::from_ptr(self.pointer.as_ref().name) };
        name.to_str().map_err(Error::invalid_data)
    }

    #[inline]
    pub fn texture(&self) -> &DynamicImage {
        unsafe {
            let pointer = self.pointer.as_ref().rendererObject as *mut DynamicImage;
            pointer.as_ref().unwrap()
        }
    }
}

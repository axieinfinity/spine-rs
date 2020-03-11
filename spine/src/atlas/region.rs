use std::{marker::PhantomData, ptr::NonNull};

use spine_sys::{spAtlasPage, spAtlasRegion};

use super::page::AtlasPage;

#[repr(transparent)]
pub struct AtlasRegion<'atlas> {
    pub(crate) pointer: NonNull<spAtlasRegion>,
    pub(crate) _marker: PhantomData<&'atlas ()>,
}

impl<'a> AtlasRegion<'a> {
    pub fn page(&self) -> AtlasPage<'a> {
        let pointer = unsafe { self.pointer.as_ref().page as *mut spAtlasPage };

        AtlasPage {
            pointer: NonNull::new(pointer).unwrap(),
            _marker: PhantomData,
        }
    }
}

use std::{marker::PhantomData, ptr::NonNull};

use spine_sys::{spAtlasPage, spAtlasRegion};

use super::page::AtlasPage;

#[repr(transparent)]
pub struct AtlasRegion<'atlas>(
    pub(crate) NonNull<spAtlasRegion>,
    pub(crate) PhantomData<&'atlas ()>,
);

impl<'a> AtlasRegion<'a> {
    pub fn page(&self) -> AtlasPage<'a> {
        let pointer = unsafe { self.0.as_ref().page as *mut spAtlasPage };
        let pointer = NonNull::new(pointer).unwrap();
        AtlasPage(pointer, PhantomData)
    }
}

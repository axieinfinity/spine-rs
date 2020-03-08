use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr::NonNull;

use spine_sys::*;

use super::error::{Error, Result};

#[repr(transparent)]
pub struct Atlas(NonNull<spAtlas>);

impl Atlas {
    pub fn from_file(path: &str) -> Result<Self> {
        let path = CString::new(path)?;
        let pointer = unsafe { spAtlas_createFromFile(path.as_ptr(), std::ptr::null_mut()) };
        let atlas = NonNull::new(pointer).ok_or(Error::NullPointer)?;
        Ok(Self(atlas))
    }

    pub fn pages(&self) -> PageIter {
        let page = unsafe { self.0.as_ref().pages };
        PageIter::new(page)
    }
}

impl Drop for Atlas {
    fn drop(&mut self) {
        unsafe { spAtlas_dispose(self.0.as_ptr()) }
    }
}

#[repr(transparent)]
pub struct AtlasPage(NonNull<spAtlasPage>);

pub struct PageIter<'a> {
    page: *mut spAtlasPage,
    _marker: PhantomData<&'a ()>,
}

impl<'a> PageIter<'a> {
    fn new(page: *mut spAtlasPage) -> Self {
        Self {
            page,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for PageIter<'a> {
    type Item = &'a AtlasPage;

    fn next(&mut self) -> Option<Self::Item> {
        NonNull::new(self.page).map(|page| unsafe {
            self.page = page.as_ref().next;
            &*(page.as_ptr() as *mut AtlasPage)
        })
    }
}

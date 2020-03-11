use std::{ffi::CString, marker::PhantomData, ptr::NonNull, rc::Rc};

use spine_sys::{spAtlas, spAtlasPage, spAtlas_createFromFile, spAtlas_dispose};

use crate::{
    error::{Error, NullPointerError},
    result::Result,
};

use super::page::AtlasPage;

#[repr(transparent)]
pub struct Atlas {
    pub(crate) pointer: NonNull<spAtlas>,
}

impl Atlas {
    pub fn from_file(path: &str) -> Result<Rc<Self>> {
        let path = CString::new(path)?;
        let pointer = unsafe { spAtlas_createFromFile(path.as_ptr(), std::ptr::null_mut()) };

        Ok(Rc::new(Self {
            pointer: NonNull::new(pointer).ok_or(Error::invalid_data(NullPointerError))?,
        }))
    }

    pub fn pages(&self) -> PageIter {
        let page = unsafe { self.pointer.as_ref().pages };
        PageIter::new(page)
    }
}

impl Drop for Atlas {
    fn drop(&mut self) {
        unsafe { spAtlas_dispose(self.pointer.as_ptr()) }
    }
}

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
    type Item = AtlasPage<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        NonNull::new(self.page).map(|pointer| unsafe {
            self.page = pointer.as_ref().next;
            AtlasPage(pointer, PhantomData)
        })
    }
}

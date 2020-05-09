use std::{marker::PhantomData, path::Path, ptr::NonNull, rc::Rc};

use spine_sys::{spAtlas, spAtlasPage, spAtlas_createFromFile, spAtlas_dispose};

use crate::{
    error::{Error, NullPointerError},
    render::Renderer,
    result::Result,
    util,
};

use super::page::AtlasPage;

#[repr(transparent)]
pub struct Atlas {
    pub(crate) pointer: NonNull<spAtlas>,
}

impl Atlas {
    pub(crate) fn new(path: impl AsRef<Path>) -> Result<Rc<Self>> {
        let path = util::c_path(path)?;
        let pointer = unsafe { spAtlas_createFromFile(path.as_ptr(), std::ptr::null_mut()) };

        Ok(Rc::new(Self {
            pointer: NonNull::new(pointer).ok_or(Error::invalid_data(NullPointerError))?,
        }))
    }

    pub fn pages(&self) -> PageIter {
        let page = unsafe { self.pointer.as_ref().pages };
        PageIter::new(page)
    }

    pub(crate) fn build_textures<R: Renderer>(&self, renderer: &mut R) -> Result<()> {
        for page in self.pages() {
            let texture = renderer.build_texture(page.texture())?;
            renderer.add_texture(page.id(), texture);
        }

        Ok(())
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

            AtlasPage {
                pointer,
                _marker: PhantomData,
            }
        })
    }
}

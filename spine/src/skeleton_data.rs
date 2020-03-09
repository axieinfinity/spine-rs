use std::{ffi::CString, marker::PhantomData, ptr::NonNull};

use spine_sys::{spSkeletonData, spSkeletonData_dispose, spSkeletonJson_readSkeletonDataFile};

use super::{
    error::{Error, NullPointerError},
    result::Result,
    skeleton_json::SkeletonJson,
};

#[repr(transparent)]
pub struct SkeletonData<'atlas>(
    pub(crate) NonNull<spSkeletonData>,
    pub(crate) PhantomData<&'atlas ()>,
);

impl<'a> SkeletonData<'a> {
    pub fn from_json_file(path: &str, skeleton_json: &SkeletonJson<'a>) -> Result<Self> {
        let path = CString::new(path)?;
        let pointer =
            unsafe { spSkeletonJson_readSkeletonDataFile(skeleton_json.0.as_ptr(), path.as_ptr()) };
        let pointer = NonNull::new(pointer).ok_or(Error::invalid_data(NullPointerError))?;
        Ok(SkeletonData(pointer, PhantomData))
    }
}

impl<'a> Drop for SkeletonData<'a> {
    fn drop(&mut self) {
        unsafe { spSkeletonData_dispose(self.0.as_ptr()) }
    }
}

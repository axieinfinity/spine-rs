use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr::NonNull;

use spine_sys::{spSkeletonData, spSkeletonData_dispose, spSkeletonJson_readSkeletonDataFile};

use super::error::Error;
use super::result::Result;
use super::skeleton_json::SkeletonJson;

#[repr(transparent)]
pub struct SkeletonData<'a>(
    pub(crate) NonNull<spSkeletonData>,
    pub(crate) PhantomData<&'a ()>,
);

impl<'a> SkeletonData<'a> {
    pub fn from_json_file(path: &str, skeleton_json: &SkeletonJson<'a>) -> Result<Self> {
        let path = CString::new(path)?;
        let pointer =
            unsafe { spSkeletonJson_readSkeletonDataFile(skeleton_json.0.as_ptr(), path.as_ptr()) };
        let pointer = NonNull::new(pointer).ok_or(Error::NullPointer)?;
        Ok(SkeletonData(pointer, PhantomData))
    }
}

impl<'a> Drop for SkeletonData<'a> {
    fn drop(&mut self) {
        unsafe { spSkeletonData_dispose(self.0.as_ptr()) }
    }
}

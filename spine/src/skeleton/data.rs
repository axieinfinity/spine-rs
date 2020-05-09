use std::{path::Path, ptr::NonNull, rc::Rc};

use spine_sys::{spSkeletonData, spSkeletonData_dispose, spSkeletonJson_readSkeletonDataFile};

use crate::{
    error::{Error, NullPointerError},
    result::Result,
    util,
};

use super::json::SkeletonJson;

pub struct SkeletonData {
    pub(crate) pointer: NonNull<spSkeletonData>,
}

impl SkeletonData {
    pub fn from_json_file(path: impl AsRef<Path>, skeleton_json: SkeletonJson) -> Result<Rc<Self>> {
        let path = util::c_path(path)?;

        let pointer = unsafe {
            spSkeletonJson_readSkeletonDataFile(skeleton_json.pointer.as_ptr(), path.as_ptr())
        };

        Ok(Rc::new(SkeletonData {
            pointer: NonNull::new(pointer).ok_or(Error::invalid_data(NullPointerError))?,
        }))
    }
}

impl Drop for SkeletonData {
    fn drop(&mut self) {
        unsafe { spSkeletonData_dispose(self.pointer.as_ptr()) }
    }
}

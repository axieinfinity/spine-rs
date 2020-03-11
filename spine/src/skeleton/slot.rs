use std::{marker::PhantomData, ptr::NonNull};

use spine_sys::{spBone, spSlot};

use super::{attachment::Attachment, bone::Bone};

#[repr(transparent)]
pub struct Slot<'skel>(
    pub(crate) NonNull<spSlot>,
    pub(crate) PhantomData<&'skel ()>,
);

impl<'a> Slot<'a> {
    pub fn bone(&self) -> Bone<'a> {
        let pointer = unsafe { self.0.as_ref().bone as *mut spBone };
        let pointer = NonNull::new(pointer).unwrap();
        Bone(pointer, PhantomData)
    }

    pub fn attachment(&self) -> Option<Attachment<'a>> {
        let pointer = unsafe { self.0.as_ref().attachment };
        Attachment::new(pointer).ok()
    }
}

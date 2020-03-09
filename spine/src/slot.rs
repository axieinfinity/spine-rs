use std::{marker::PhantomData, ptr::NonNull};

use spine_sys::spSlot;

use super::attachment::Attachment;

#[repr(transparent)]
pub struct Slot<'skel>(
    pub(crate) NonNull<spSlot>,
    pub(crate) PhantomData<&'skel ()>,
);

impl<'skel> Slot<'skel> {
    pub fn attachment(&self) -> Option<Attachment<'skel>> {
        let pointer = unsafe { self.0.as_ref().attachment };
        Attachment::from_pointer(pointer).ok()
    }
}

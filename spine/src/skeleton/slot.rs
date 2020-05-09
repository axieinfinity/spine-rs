use std::{marker::PhantomData, ptr::NonNull};

use spine_sys::{spSlot, spSlot_setAttachment};

use super::{attachment::Attachment, bone::Bone};

#[repr(transparent)]
pub struct Slot<'skel>(
    pub(crate) NonNull<spSlot>,
    pub(crate) PhantomData<&'skel ()>,
);

impl<'a> Slot<'a> {
    pub fn bone(&self) -> Bone<'a> {
        let pointer = unsafe { self.0.as_ref().bone };
        let pointer = NonNull::new(pointer).unwrap();
        Bone(pointer, PhantomData)
    }

    pub fn attachment(&self) -> Option<Attachment<'a>> {
        let pointer = unsafe { self.0.as_ref().attachment };
        Attachment::new(pointer).ok()
    }

    pub fn set_attachment(&mut self, attachment: Option<Attachment>) {
        let pointer = match attachment {
            Some(Attachment::Region(region)) => region.0.cast().as_ptr(),
            Some(Attachment::Mesh(mesh)) => mesh.0.cast().as_ptr(),
            Some(Attachment::Other) | None => std::ptr::null_mut(),
        };

        unsafe {
            spSlot_setAttachment(self.0.as_ptr(), pointer);
        }
    }
}

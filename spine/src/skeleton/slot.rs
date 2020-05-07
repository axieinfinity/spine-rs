use std::{marker::PhantomData, ptr::NonNull};

use spine_sys::{spBone, spSlot, spSlot_setAttachment};

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

    pub fn set_attachment(&mut self, attachment: Option<Attachment>) {
        let pointer = match attachment {
            Some(Attachment::Other) => None,
            Some(Attachment::Mesh(mesh_attachment)) => Some(mesh_attachment.0.cast()),
            Some(Attachment::Region(region_attachment)) => Some(region_attachment.0.cast()),
            None => None,
        };

        match pointer {
            Some(mut pointer) => unsafe { spSlot_setAttachment(self.0.as_mut(), pointer.as_mut()) },
            None => unsafe { spSlot_setAttachment(self.0.as_mut(), 0 as *mut _) },
        }
    }
}

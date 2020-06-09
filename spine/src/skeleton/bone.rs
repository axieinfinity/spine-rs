use std::{marker::PhantomData, ptr::NonNull};

use spine_sys::spBone;

#[repr(transparent)]
pub struct Bone<'skel> {
    pub(crate) pointer: NonNull<spBone>,
    pub(crate) _marker: PhantomData<&'skel ()>,
}

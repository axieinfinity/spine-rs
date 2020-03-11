use std::{marker::PhantomData, ptr::NonNull};

use spine_sys::spBone;

#[repr(transparent)]
pub struct Bone<'skel>(
    pub(crate) NonNull<spBone>,
    pub(crate) PhantomData<&'skel ()>,
);

use std::ptr::NonNull;

use spine_sys::spAtlasPage;

#[repr(transparent)]
pub struct AtlasPage(pub(crate) NonNull<spAtlasPage>);

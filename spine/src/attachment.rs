#![allow(non_upper_case_globals)]

use std::{marker::PhantomData, os::raw::c_int, ptr::NonNull, slice};

use spine_sys::{
    spAtlasRegion, spAttachment, spAttachmentType_SP_ATTACHMENT_MESH,
    spAttachmentType_SP_ATTACHMENT_REGION, spBone, spMeshAttachment, spRegionAttachment,
    spRegionAttachment_computeWorldVertices, spSlot, spVertexAttachment,
    spVertexAttachment_computeWorldVertices,
};

use super::{
    atlas::AtlasRegion,
    bone::Bone,
    error::{Error, NullPointerError},
    result::Result,
    slot::Slot,
};

pub enum Attachment<'skel> {
    Region(RegionAttachment<'skel>),
    Mesh(MeshAttachment<'skel>),
    Other,
}

#[repr(transparent)]
pub struct RegionAttachment<'skel>(
    pub(crate) NonNull<spRegionAttachment>,
    pub(crate) PhantomData<&'skel ()>,
);

#[repr(transparent)]
pub struct MeshAttachment<'skel>(
    pub(crate) NonNull<spMeshAttachment>,
    pub(crate) PhantomData<&'skel ()>,
);

impl<'a> Attachment<'a> {
    pub(crate) fn new(pointer: *mut spAttachment) -> Result<Self> {
        let pointer = NonNull::new(pointer).ok_or(Error::invalid_input(NullPointerError))?;

        Ok(match unsafe { pointer.as_ref().type_ } {
            spAttachmentType_SP_ATTACHMENT_REGION => {
                Attachment::Region(RegionAttachment(pointer.cast(), PhantomData))
            }

            spAttachmentType_SP_ATTACHMENT_MESH => {
                Attachment::Mesh(MeshAttachment(pointer.cast(), PhantomData))
            }

            _ => Attachment::Other,
        })
    }
}

impl<'a> MeshAttachment<'a> {
    pub fn region(&self) -> AtlasRegion<'a> {
        let pointer = unsafe { self.0.as_ref().rendererObject as *mut spAtlasRegion };

        AtlasRegion {
            pointer: NonNull::new(pointer).unwrap(),
            _marker: PhantomData,
        }
    }

    pub fn triangles(&self) -> &[u16] {
        unsafe {
            let mesh = self.0.as_ref();
            slice::from_raw_parts(mesh.triangles as *const u16, mesh.trianglesCount as usize)
        }
    }

    pub fn uvs(&self) -> &[f32] {
        unsafe {
            slice::from_raw_parts(self.0.as_ref().uvs as *const f32, self.world_vertices_len())
        }
    }

    pub fn world_vertices_len(&self) -> usize {
        unsafe { self.0.as_ref().super_.worldVerticesLength as usize }
    }

    pub fn compute_world_vertices(
        &self,
        slot: &Slot,
        start: usize,
        count: usize,
        vertices: &mut Vec<f32>,
        offset: usize,
        stride: usize,
    ) {
        unsafe {
            spVertexAttachment_computeWorldVertices(
                &self.0.as_ref().super_ as *const _ as *mut spVertexAttachment,
                slot.0.as_ptr() as *mut spSlot,
                start as c_int,
                count as c_int,
                vertices.as_mut_ptr(),
                offset as c_int,
                stride as c_int,
            );
        }
    }
}

impl<'a> RegionAttachment<'a> {
    pub fn region(&self) -> AtlasRegion<'a> {
        let pointer = unsafe { self.0.as_ref().rendererObject as *mut spAtlasRegion };

        AtlasRegion {
            pointer: NonNull::new(pointer).unwrap(),
            _marker: PhantomData,
        }
    }

    pub fn uvs(&self) -> &[f32] {
        unsafe { &self.0.as_ref().uvs }
    }

    pub fn compute_world_vertices(
        &self,
        bone: &Bone,
        vertices: &mut Vec<f32>,
        offset: usize,
        stride: usize,
    ) {
        unsafe {
            spRegionAttachment_computeWorldVertices(
                self.0.as_ptr(),
                bone.0.as_ptr() as *mut spBone,
                vertices.as_mut_ptr(),
                offset as c_int,
                stride as c_int,
            );
        }
    }
}

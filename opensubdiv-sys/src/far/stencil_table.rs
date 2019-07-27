use crate::vtr::types::*;

#[repr(C)]
pub struct Stencil {
    pub(crate) size: *const i32,
    pub(crate) indices: *const Index,
    pub(crate) weights: *const f32,
}

impl Stencil {
    pub fn size(&self) -> *const i32 {
        self.size
    }
    pub fn indices(&self) -> *const Index {
        self.indices
    }
    pub fn weights(&self) -> *const f32 {
        self.weights
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StencilTable_obj {
    _unused: [u8; 0],
}
pub type StencilTablePtr = *mut StencilTable_obj;

extern "C" {
    pub fn StencilTable_destroy(st: StencilTablePtr);
    /// Returns the number of stencils in the table
    pub fn StencilTable_GetNumStencils(st: StencilTablePtr) -> i32;
    /// Returns the number of control vertices indexed in the table
    pub fn StencilTable_GetNumControlVertices(st: StencilTablePtr) -> i32;
    /// Returns a Stencil at index i in the table
    pub fn StencilTable_GetStencil(st: StencilTablePtr, i: Index) -> Stencil;
    /// Returns the number of control vertices of each stencil in the table
    pub fn StencilTable_GetSizes(st: StencilTablePtr) -> IntVectorRef;
    /// Returns the offset to a given stencil (factory may leave empty)
    pub fn StencilTable_GetOffsets(st: StencilTablePtr) -> IndexVectorRef;
    /// Returns the indices of the control vertices
    pub fn StencilTable_GetControlIndices(
        st: StencilTablePtr,
    ) -> IndexVectorRef;
    /// Returns the stencil interpolation weights
    pub fn StencilTable_GetWeights(st: StencilTablePtr) -> FloatVectorRef;
}

use crate::vtr::types::*;

#[repr(C)]
pub struct Stencil {
    size: *const i32,
    indices: *const Index,
    weights: *const f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StencilTable_obj {
    _unused: [u8; 0],
}
pub type StencilTablePtr = *mut StencilTable_obj;

extern "C" {
    /// \brief Returns the number of stencils in the table
    pub fn StencilTable_GetNumStencils(st: StencilTablePtr) -> i32;
    /// \brief Returns the number of control vertices indexed in the table
    pub fn StencilTable_GetNumControlVertices(st: StencilTablePtr) -> i32;
    /// \brief Returns a Stencil at index i in the table
    pub fn StencilTable_GetStencil(st: StencilTablePtr, i: Index) -> Stencil;
    /// \brief Returns the number of control vertices of each stencil in the table
    pub fn StencilTable_GetSizes(st: StencilTablePtr) -> IntVectorRef;
    /// \brief Returns the offset to a given stencil (factory may leave empty)
    pub fn StencilTable_GetOffsets(st: StencilTablePtr) -> IndexVectorRef;
    /// \brief Returns the indices of the control vertices
    pub fn StencilTable_GetControlIndices(st: StencilTablePtr) -> IndexVectorRef;
    /// \brief Returns the stencil interpolation weights
    pub fn StencilTable_GetWeights(st: StencilTablePtr) -> FloatVectorRef;
}
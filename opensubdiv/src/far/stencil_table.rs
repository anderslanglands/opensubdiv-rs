use opensubdiv_sys as sys;

use crate::Index;

/// Allows access to a single stencil in the [StencilTable]
pub struct Stencil<'a> {
    indices: &'a [Index],
    weights: &'a [f32],
}

/// Table of subdivision stencils.
///
/// Stencils are the most direct method of evaluation of locations on the limit
/// of a surface. Every point of a limit surface can be computed by linearly
/// blending a collection of coarse control vertices.
/// A stencil assigns a series of control vertex indices with a blending weight
/// that corresponds to a unique parametric location of the limit surface. When
/// the control vertices move in space, the limit location can be very
/// efficiently recomputed simply by applying the blending weights to the
/// series of coarse control vertices.
pub struct StencilTable {
    pub(crate) ptr: sys::far::StencilTablePtr,
}

impl Drop for StencilTable {
    fn drop(&mut self) {
        unsafe { sys::far::StencilTable_destroy(self.ptr) }
    }
}

impl StencilTable {
    /// Returns the number of stencils in the table
    pub fn get_num_stencils(&self) -> i32 {
        unsafe { sys::far::StencilTable_GetNumStencils(self.ptr) }
    }

    /// Returns the number of control vertices indexed in the table
    pub fn get_num_control_vertices(&self) -> i32 {
        unsafe { sys::far::StencilTable_GetNumControlVertices(self.ptr) }
    }

    /// Returns a Stencil at index i in the table
    pub fn get_stencil(&self, i: Index) -> Option<Stencil> {
        if i < Index(0) || i >= Index(self.get_num_stencils()) {
            None
        } else {
            unsafe {
                let stencil = sys::far::StencilTable_GetStencil(self.ptr, i);
                Some(Stencil {
                    indices: std::slice::from_raw_parts(
                        stencil.indices(),
                        *stencil.size() as usize,
                    ),
                    weights: std::slice::from_raw_parts(
                        stencil.weights(),
                        *stencil.size() as usize,
                    ),
                })
            }
        }
    }

    /// Returns the number of control vertices of each stencil in the table
    pub fn get_sizes(&self) -> &[i32] {
        unsafe {
            let vr = sys::far::StencilTable_GetSizes(self.ptr);
            std::slice::from_raw_parts(vr.data(), vr.size())
        }
    }

    /// Returns the offset to a given stencil (factory may leave empty)
    pub fn get_offsets(&self) -> &[Index] {
        unsafe {
            let vr = sys::far::StencilTable_GetOffsets(self.ptr);
            std::slice::from_raw_parts(vr.data(), vr.size())
        }
    }

    /// Returns the indices of the control vertices
    pub fn get_control_indices(&self) -> &[Index] {
        unsafe {
            let vr = sys::far::StencilTable_GetControlIndices(self.ptr);
            std::slice::from_raw_parts(vr.data(), vr.size())
        }
    }

    /// Returns the stencil interpolation weights
    pub fn get_weights(&self) -> &[f32] {
        unsafe {
            let vr = sys::far::StencilTable_GetWeights(self.ptr);
            std::slice::from_raw_parts(vr.data(), vr.size())
        }
    }
}

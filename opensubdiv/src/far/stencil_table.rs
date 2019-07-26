use opensubdiv_sys as sys;

pub struct StencilTable {
    pub(crate) ptr: sys::far::StencilTablePtr,
}

impl StencilTable {
    pub fn get_num_stencils(&self) -> i32 {
        unsafe {
            sys::far::StencilTable_GetNumStencils(self.ptr)
        }
    }
}

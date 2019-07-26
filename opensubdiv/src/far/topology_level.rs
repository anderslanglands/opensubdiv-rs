use opensubdiv_sys as sys;

pub struct TopologyLevel {
    pub(crate) ptr: sys::far::TopologyLevelPtr,
}

impl TopologyLevel {
    pub fn get_num_vertices(&self) -> i32 {
        unsafe {
            sys::far::TopologyLevel_GetNumVertices(self.ptr)
        }
    }
}
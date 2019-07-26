use opensubdiv_sys as sys;

pub use sys::topology_refiner::{UniformOptions, UniformOptionsBuilder};

use super::topology_level::TopologyLevel;

pub struct TopologyRefiner {
    pub(crate) ptr: sys::far::TopologyRefinerPtr,
}

impl TopologyRefiner {
    pub fn refine_uniform(&mut self, options: UniformOptions) {
        unsafe {
            sys::far::TopologyRefiner_RefineUniform(self.ptr, options);
        }
    }

    pub fn get_max_level(&self) -> i32 {
        unsafe { sys::far::TopologyRefiner_GetMaxLevel(self.ptr) }
    }

    pub fn get_level(&self, level: i32) -> Option<TopologyLevel> {
        if level > self.get_max_level() || level < 0 {
            None
        } else {
            let ptr =
                unsafe { sys::far::TopologyRefiner_GetLevel(self.ptr, level) };
            if ptr.is_null() {
                panic!("TopologyRefiner_GetLevel() returned null");
            }
            Some(TopologyLevel { ptr })
        }
    }
}

impl Drop for TopologyRefiner {
    fn drop(&mut self) {
        unsafe {
            sys::far::TopologyRefiner_destroy(self.ptr);
        }
    }
}

use opensubdiv_sys as sys;

pub use crate::sdc;

pub use sys::topology_refiner::{UniformOptions, UniformOptionsBuilder};

use super::topology_level::TopologyLevel;

pub fn uniform_options() -> UniformOptionsBuilder {
    UniformOptionsBuilder::new()
}

pub struct TopologyRefiner {
    pub(crate) ptr: sys::far::TopologyRefinerPtr,
}

impl TopologyRefiner {
    /// Returns the subdivision scheme
    pub fn get_scheme_type(&self) -> sdc::SchemeType {
        unsafe { sys::far::TopologyRefiner_GetSchemeType(self.ptr) }
    }

    /// Returns the subdivision options
    pub fn get_scheme_options(&self) -> sdc::Options {
        unsafe { sys::far::TopologyRefiner_GetSchemeOptions(self.ptr) }
    }

    /// Returns true if uniform refinement has been applied
    pub fn is_uniform(&self) -> bool {
        unsafe { sys::far::TopologyRefiner_IsUniform(self.ptr) }
    }

    /// Returns the number of refinement levels
    pub fn get_num_levels(&self) -> i32 {
        unsafe { sys::far::TopologyRefiner_GetNumLevels(self.ptr) }
    }

    /// Returns the maximum vertex valence in all levels
    pub fn get_max_valence(&self) -> i32 {
        unsafe { sys::far::TopologyRefiner_GetMaxValence(self.ptr) }
    }

    /// Returns true if faces have been tagged as holes
    pub fn has_holes(&self) -> bool {
        unsafe { sys::far::TopologyRefiner_HasHoles(self.ptr) }
    }

    /// Returns the total number of vertices in all levels
    pub fn get_num_vertices_total(&self) -> i32 {
        unsafe { sys::far::TopologyRefiner_GetNumVerticesTotal(self.ptr) }
    }

    /// Returns the total number of edges in all levels
    pub fn get_num_edges_total(&self) -> i32 {
        unsafe { sys::far::TopologyRefiner_GetNumEdgesTotal(self.ptr) }
    }

    /// Returns the total number of faces in all levels
    pub fn get_num_faces_total(&self) -> i32 {
        unsafe { sys::far::TopologyRefiner_GetNumFacesTotal(self.ptr) }
    }

    /// Returns the total number of face vertices in all levels
    pub fn get_num_face_vertices_total(&self) -> i32 {
        unsafe { sys::far::TopologyRefiner_GetNumFaceVerticesTotal(self.ptr) }
    }

    /// Returns the highest level of refinement
    pub fn get_max_level(&self) -> i32 {
        unsafe { sys::far::TopologyRefiner_GetMaxLevel(self.ptr) }
    }

    /// Returns a handle to access data specific to a particular level
    pub fn get_level(&self, level: i32) -> Option<TopologyLevel> {
        if level > self.get_max_level() || level < 0 {
            None
        } else {
            let ptr =
                unsafe { sys::far::TopologyRefiner_GetLevel(self.ptr, level) };
            if ptr.is_null() {
                None
            } else {
                Some(TopologyLevel {
                    ptr,
                    refiner: std::marker::PhantomData,
                })
            }
        }
    }

    /// Refine the topology uniformly
    ///
    /// This method applies uniform refinement to the level specified in the
    /// given UniformOptions.
    ///
    /// Note the impact of the UniformOption to generate fullTopologyInLastLevel
    /// and be sure it is assigned to satisfy the needs of the resulting refinement.
    ///
    /// * `options` - Options controlling uniform refinement
    ///
    pub fn refine_uniform(&mut self, options: UniformOptions) {
        unsafe {
            sys::far::TopologyRefiner_RefineUniform(self.ptr, options);
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

use opensubdiv_sys as sys;
use super::topology_refiner::TopologyRefiner;
use super::stencil_table::StencilTable;

pub use sys::far::stencil_table_factory::{Options, OptionsBuilder};

pub fn create(refiner: &TopologyRefiner, options: sys::far::stencil_table_factory::Options) -> StencilTable {
    let ptr = unsafe {
        sys::far::StencilTableFactory_Create(refiner.ptr, options)
    };

    if ptr.is_null() {
        panic!("StencilTableFactory_Create() returned null");
    }

    StencilTable {
        ptr
    }
}
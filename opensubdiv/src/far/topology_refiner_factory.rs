use super::topology_descriptor::TopologyDescriptor;
use super::topology_refiner::TopologyRefiner;
use opensubdiv_sys as sys;
pub use sys::far::topology_refiner_factory::Options;

pub fn create(
    descriptor: TopologyDescriptor,
    options: Options,
) -> TopologyRefiner {
    let ptr = unsafe {
        let desc = descriptor.into();
        sys::far::TopologyRefinerFactory_TopologyDescriptor_Create(
            &desc as *const sys::far::TopologyDescriptor,
            options,
        )
    };

    if ptr.is_null() {
        panic!("TopologyRefinerFactory_TopologyDescriptor_Create returned a null pointer");
    }

    TopologyRefiner { ptr }
}

use super::topology_descriptor::TopologyDescriptor;
use super::topology_refiner::TopologyRefiner;
pub use sys::far::topology_refiner_factory::Options;

use crate::Error;
type Result<T, E = Error> = std::result::Result<T, E>;

use opensubdiv_sys as sys;

/// Instantiates a TopologyRefiner from client-provided topological
/// representation.
///
///  If only the face-vertices topological relationships are specified
///  with this factory, edge relationships have to be inferred, which
///  requires additional processing. If the client topological rep can
///  provide this information, it is highly recommended to do so.
///
/// * `mesh` - Client's topological representation (or a converter)
/// * `options` - Options controlling the creation of the TopologyRefiner
pub fn create(
    descriptor: TopologyDescriptor,
    options: Options,
) -> Result<TopologyRefiner> {
    let ptr = unsafe {
        let desc = descriptor.into();
        sys::far::TopologyRefinerFactory_TopologyDescriptor_Create(
            &desc as *const sys::far::TopologyDescriptor,
            options,
        )
    };

    if ptr.is_null() {
        Err(Error::CreateTopologyRefinerFailed)
    } else {
        Ok(TopologyRefiner { ptr })
    }
}

pub mod topology_descriptor;
pub use topology_descriptor::*;

pub mod topology_level;
pub use topology_level::*;

pub mod topology_refiner;
pub use topology_refiner::*;

pub mod topology_refiner_factory;
pub use topology_refiner_factory::*;

pub mod stencil_table;
pub use stencil_table::*;

pub mod stencil_table_factory;
pub use stencil_table_factory::*;

#[derive(Display, Debug)]
pub enum Error {
    #[display(fmt = "TopologyRefinerFactory failed to create TopologyRefiner")]
    CreateTopologyRefinerFailed,
}

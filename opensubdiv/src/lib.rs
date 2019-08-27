pub mod far;
pub mod osd;
pub mod sdc;

pub use opensubdiv_sys::vtr::Index;

#[macro_use]
extern crate derive_more;

#[derive(Display, Debug)]
pub enum Error {
    #[display(fmt = "TopologyRefinerFactory failed to create TopologyRefiner")]
    CreateTopologyRefinerFailed,
    #[display(fmt = "Stencil evaluation failed")]
    EvalStencilsFailed,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

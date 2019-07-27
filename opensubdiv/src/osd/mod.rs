pub mod buffer_descriptor;
pub use buffer_descriptor::*;

pub mod cpu_evaluator;
pub use cpu_evaluator::*;

pub mod cpu_vertex_buffer;
pub use cpu_vertex_buffer::*;

#[derive(Display, Debug)]
pub enum Error {
    #[display(fmt = "Stencil evaluation failed")]
    EvalStencilsFailed,
}

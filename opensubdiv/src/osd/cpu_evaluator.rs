use super::buffer_descriptor::BufferDescriptor;
use super::cpu_vertex_buffer::CpuVertexBuffer;
use crate::far::StencilTable;

use opensubdiv_sys as sys;

// FIXME: this is unsafe because we're mutating dst_buffer but pretending
// we're not
pub unsafe fn eval_stencils(
    src_buffer: &CpuVertexBuffer,
    src_desc: BufferDescriptor,
    dst_buffer: &CpuVertexBuffer,
    dst_desc: BufferDescriptor,
    stencil_table: &StencilTable,
) -> bool {
    sys::osd::CpuEvaluator_EvalStencils(
        src_buffer.ptr,
        src_desc,
        dst_buffer.ptr,
        dst_desc,
        stencil_table.ptr,
    )
}

use opensubdiv_sys as sys;

/// Concrete vertex buffer class for CUDA subdivision.
///
/// [CudaVertexBuffer] implements the VertexBufferInterface. An instance
/// of this buffer class can be passed to [CudaEvaluator]
///
pub struct CudaVertexBuffer {
    pub(crate) ptr: sys::osd::CudaVertexBufferPtr,
}

impl Drop for CudaVertexBuffer {
    fn drop(&mut self) {
        unsafe { sys::osd::CudaVertexBuffer_destroy(self.ptr) }
    }
}

impl CudaVertexBuffer {
    pub fn new(num_elements: i32, num_vertices: i32) -> CudaVertexBuffer {
        let ptr = unsafe {
            sys::osd::CudaVertexBuffer_Create(
                num_elements,
                num_vertices,
                std::ptr::null(),
            )
        };
        if ptr.is_null() {
            panic!("CudaVertexBuffer_Create returned null");
        }

        CudaVertexBuffer { ptr }
    }

    /// Returns how many elements defined in this vertex buffer.
    pub fn get_num_elements(&self) -> i32 {
        unsafe { sys::osd::CudaVertexBuffer_GetNumElements(self.ptr) }
    }

    /// Returns how many vertices allocated in this vertex buffer.
    pub fn get_num_vertices(&self) -> i32 {
        unsafe { sys::osd::CudaVertexBuffer_GetNumVertices(self.ptr) }
    }

    /// Get the contents of this vertex buffer as a slice of f32.
    pub fn bind_cuda_buffer(&self) -> &[f32] {
        let ptr = unsafe { sys::osd::CudaVertexBuffer_BindCudaBuffer(self.ptr) };
        if ptr.is_null() {
            panic!("CudaVertexBuffer_BindCudaBuffer() returned null");
        }

        unsafe {
            std::slice::from_raw_parts(
                ptr,
                (self.get_num_elements() * self.get_num_vertices()) as usize,
            )
        }
    }

    /// This method is meant to be used in client code in order to provide
    /// coarse vertices data to Osd.
    pub fn update_data(
        &mut self,
        src: &[f32],
        start_vertex: i32,
        num_vertices: i32,
    ) {
        // do some basic error checking
        let num_elements = self.get_num_elements();

        if start_vertex < 0
            || (start_vertex * num_elements) as usize > src.len()
        {
            panic!(
                "Start vertex is out of range of the src slice: {} ({})",
                start_vertex,
                src.len()
            );
        }

        if num_vertices < 0
            || (num_vertices * num_elements) as usize > src.len()
        {
            panic!(
                "num vertices is out of range of the src slice: {} ({})",
                num_vertices,
                src.len()
            );
        }

        unsafe {
            sys::osd::CudaVertexBuffer_UpdateData(
                self.ptr,
                src.as_ptr(),
                start_vertex,
                num_vertices,
                std::ptr::null(),
            );
        }
    }
}


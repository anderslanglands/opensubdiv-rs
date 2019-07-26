use opensubdiv_sys as sys;

pub struct CpuVertexBuffer {
    pub(crate) ptr: sys::osd::CpuVertexBufferPtr,
}

impl CpuVertexBuffer {
    pub fn new(num_elements: i32, num_vertices: i32) -> CpuVertexBuffer {
        let ptr = unsafe {
            sys::osd::CpuVertexBuffer_Create(
                num_elements,
                num_vertices,
                std::ptr::null(),
            )
        };
        if ptr.is_null() {
            panic!("CpuVertexBuffer_Create returned null");
        }

        CpuVertexBuffer { ptr }
    }

    pub fn get_num_elements(&self) -> i32 {
        unsafe { sys::osd::CpuVertexBuffer_GetNumElements(self.ptr) }
    }

    pub fn get_num_vertices(&self) -> i32 {
        unsafe { sys::osd::CpuVertexBuffer_GetNumVertices(self.ptr) }
    }

    pub fn bind_cpu_buffer(&self) -> &[f32] {
        let ptr = unsafe { sys::osd::CpuVertexBuffer_BindCpuBuffer(self.ptr) };
        if ptr.is_null() {
            panic!("CpuVertexBuffer_BindCpuBuffer() returned null");
        }

        unsafe {
            std::slice::from_raw_parts(
                ptr,
                (self.get_num_elements() * self.get_num_vertices()) as usize,
            )
        }
    }

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
            sys::osd::CpuVertexBuffer_UpdateData(
                self.ptr,
                src.as_ptr(),
                start_vertex,
                num_vertices,
                std::ptr::null(),
            );
        }
    }
}

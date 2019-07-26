use opensubdiv::{far, osd, sdc};

fn main() {
    let vertices = [
        -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5,
        0.5, -0.5, 0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5,
    ];
    let num_vertices = 8;
    let num_faces = 6;

    let verts_per_face = [4, 4, 4, 4, 4, 4];

    let vert_indices = [
        0, 1, 3, 2, 2, 3, 5, 4, 4, 5, 7, 6, 6, 7, 1, 0, 1, 7, 5, 3, 6, 0, 2, 4,
    ];

    let descriptor = far::TopologyDescriptor::new(
        num_vertices,
        num_faces,
        &verts_per_face,
        &vert_indices,
    );

    let mut refiner = far::topology_refiner_factory::create(
        descriptor,
        far::topology_refiner_factory::Options::new(
            sdc::SchemeType::Catmark,
            sdc::OptionsBuilder::new()
                .vtx_boundary_interpolation(
                    sdc::VtxBoundaryInterpolation::EdgeOnly,
                )
                .build(),
        ),
    );

    refiner.refine_uniform(
        far::UniformOptionsBuilder::new()
            .refinement_level(2)
            .build(),
    );

    let stencil_table = far::stencil_table_factory::create(
        &refiner,
        far::stencil_table_factory::OptionsBuilder::new()
            .generate_offsets(true)
            .generate_intermediate_levels(false)
            .build(),
    );

    let n_coarse_verts = refiner.get_level(0).unwrap().get_num_vertices();
    let n_refined_verts = stencil_table.get_num_stencils();

    // set up a buffer for primvar data
    let mut vbuffer =
        osd::CpuVertexBuffer::new(3, n_coarse_verts + n_refined_verts);

    // execution phase (every frame)
    {
        // pack the control vertices at the start of the buffer
        vbuffer.update_data(&vertices, 0, n_coarse_verts);

        let src_desc = osd::BufferDescriptor::new(0, 3, 3);
        let dst_desc = osd::BufferDescriptor::new(n_coarse_verts * 3, 3, 3);

        // launch the computation
        let result = unsafe {
            osd::eval_stencils(
                &vbuffer,
                src_desc,
                &vbuffer,
                dst_desc,
                &stencil_table,
            )
        };

        if !result {
            panic!("eval_stencils failed");
        }

        let refined_verts = vbuffer.bind_cpu_buffer();
        for v in refined_verts.chunks(3).skip(n_coarse_verts as usize) {
            println!("{:?}", v);
        }
    }
}

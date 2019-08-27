use opensubdiv::Index;
use opensubdiv::{far, osd, sdc};

#[derive(Copy, Clone)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

struct VertexSlice<'a> {
    slice: &'a [Vertex],
}

struct VertexSliceMut<'a> {
    slice: &'a mut [Vertex],
}

impl<'a> far::PrimvarBufferSrc for VertexSlice<'a> {
    const NUM_ELEMENTS: i32 = 3;

    fn as_f32(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(
                self.slice.as_ptr() as *const f32,
                self.slice.len() * 3,
            )
        }
    }
}

impl<'a> far::PrimvarBufferDst for VertexSliceMut<'a> {
    const NUM_ELEMENTS: i32 = 3;

    fn as_f32_mut(&mut self) -> &mut [f32] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.slice.as_mut_ptr() as *mut f32,
                self.slice.len() * 3,
            )
        }
    }
}

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

    // populate a descriptor with our raw data
    let descriptor = far::TopologyDescriptor::new(
        num_vertices,
        num_faces,
        &verts_per_face,
        &vert_indices,
    );

    // instantiate a TopologyRefiner from the descriptor
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
    )
    .expect("Could not create TopologyRefiner");

    let max_level = 2i32;
    // uniformly refine up to 'max level' of 2
    refiner.refine_uniform(
        far::uniform_options()
            .refinement_level(max_level as u32)
            .build(),
    );

    // Allocate a buffer for the control vertices
    let mut vbuffer = Vec::with_capacity(
        refiner.get_level(0).unwrap().get_num_vertices() as usize,
    );

    // initialize coarse mesh positions
    for v in vertices.chunks(3) {
        vbuffer.push(Vertex {
            x: v[0],
            y: v[1],
            z: v[2],
        });
    }

    // interpolate vertex primvar data
    let primvar_refiner = far::PrimvarRefiner::new(&refiner);

    let mut refined_verts = Vec::with_capacity(max_level as usize);
    refined_verts.push(vbuffer);
    for level in 1..=max_level {
        let mut dst_vec = vec![
            Vertex {
                x: 0.0,
                y: 0.0,
                z: 0.0
            };
            refiner.get_level(level).unwrap().get_num_vertices()
                as usize
        ];

        let src = unsafe {
            VertexSlice {
                slice: refined_verts
                    .get_unchecked(level as usize - 1)
                    .as_slice(),
            }
        };

        let mut dst = VertexSliceMut {
            slice: dst_vec.as_mut_slice(),
        };

        primvar_refiner.interpolate(level, &src, &mut dst);

        refined_verts.push(dst_vec);
    }

    // output an OBJ of the highest level
    let last_level = refiner.get_level(max_level).unwrap();
    let nfaces = last_level.get_num_faces();
    // print vertex positions
    for v in refined_verts.last().unwrap().iter() {
        println!("v {} {} {}", v.x, v.y, v.z);
    }

    // for f in 0..nfaces {
    //     let face_vert_indices = last_level.get_face_vertices(Index(f)).unwrap();
    for face_vert_indices in last_level.face_vertices_iter() {

        // all refined cat-clark faces should be quads
        assert!(face_vert_indices.len() == 4);
        print!("f ");
        for fv in face_vert_indices {
            print!("{} ", fv.0 + 1);
        }
        print!("\n");
    }
}

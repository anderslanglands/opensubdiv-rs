use opensubdiv_sys as sys;

/// A simple reference to raw topology data for use with TopologyRefinerFactory
///
/// TopologyDescriptor is a simple struct containing references to raw topology
/// data used to construct a TopologyRefiner.  It is not a requirement but a
/// convenience for use with TopologyRefinerFactory when mesh topology is not
/// available in an existing mesh data structure.  It should be functionally
/// complete and simple to use, but for more demanding situations, writing a
/// custom Factory is usually warranted.
pub struct TopologyDescriptor<'a> {
    num_vertices: i32,
    num_faces: i32,

    num_verts_per_face: &'a [i32],
    vert_indices_per_face: &'a [i32],

    num_creases: i32,
    crease_vertex_index_pairs: Option<&'a [i32]>,
    crease_weights: Option<&'a [f32]>,

    hole_indices: Option<&'a [i32]>,

    is_left_handed: bool,
}

impl<'a> TopologyDescriptor<'a> {
    /// Create a new [TopologyDescriptor]
    ///
    /// ## Parameters
    /// * `num_vertices` - The number of vertices in the mesh.
    /// * `num_faces` - The number of faces in the mesh.
    /// * `num_verts_per_face` - A slice containing the number of vertices for
    /// each face in the mesh.
    /// * `vert_indices_per_face` - A flat list of the vertex indices for each
    /// face in the mesh.
    pub fn new(
        num_vertices: i32,
        num_faces: i32,
        num_verts_per_face: &'a [i32],
        vert_indices_per_face: &'a [i32],
    ) -> TopologyDescriptor<'a> {
        TopologyDescriptor {
            num_vertices,
            num_faces,
            num_verts_per_face,
            vert_indices_per_face,
            num_creases: 0,
            crease_vertex_index_pairs: None,
            crease_weights: None,
            hole_indices: None,
            is_left_handed: false,
        }
    }
}

impl<'a> Into<sys::far::TopologyDescriptor> for TopologyDescriptor<'a> {
    fn into(self) -> sys::far::TopologyDescriptor {
        sys::far::TopologyDescriptor::new(
            self.num_vertices,
            self.num_faces,
            self.num_verts_per_face.as_ptr(),
            self.vert_indices_per_face.as_ptr(),
        )
    }
}

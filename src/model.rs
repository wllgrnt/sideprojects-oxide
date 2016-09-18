extern crate glium;

use vertex::Vertex;
use program;

// ============================================================
// Model
// ============================================================
/// The mesh of a single object (a triangle, a sphere, a goove...)
pub struct Model<'a> {
    /// The vertices of the triangles out of which the mesh is made
    _vertices        : Vec<Vertex>,
    /// The order in which the vertices should be drawn.
    _index_type      : glium::index::PrimitiveType,
    _indices         : Vec<u16>,
    _program         : &'a glium::Program,
    _vertex_buffer   : glium::VertexBuffer<Vertex>,
    _index_buffer    : glium::index::IndexBuffer<u16>,
}

impl<'a> Model<'a> {
    pub fn new (
        in_display    : &glium::backend::glutin_backend::GlutinFacade,
        in_vertices   : &Vec<Vertex>,
        in_index_type : &glium::index::PrimitiveType,
        in_indices    : &Vec<u16>,
        in_program    : &'a glium::Program,
    ) -> Model<'a> {
        Model {
            _vertices      : in_vertices.to_owned(),
            _index_type    : in_index_type.to_owned(),
            _indices       : in_indices.to_owned(),
            _vertex_buffer : glium::VertexBuffer::new(in_display, in_vertices).unwrap(),
            _index_buffer  : glium::index::IndexBuffer::new (
                in_display,
                *in_index_type,
                in_indices,
            ).unwrap(),
            _program       : in_program,
        }
    }

    pub fn vertex_buffer(&self) -> &glium::VertexBuffer<Vertex> {&self._vertex_buffer}
    pub fn index_buffer(&self) -> &glium::index::IndexBuffer<u16> {&self._index_buffer}
    pub fn program(&self) -> &glium::Program {&self._program}
}

pub struct DefaultModels<'a> {
    _triangle    : Model<'a>,
    _square      : Model<'a>,
    _tetrahedron : Model<'a>,
    _cube        : Model<'a>,
    _icosahedron : Model<'a>,
    _sphere      : Model<'a>,
}

impl<'a> DefaultModels<'a> {
    pub fn new (
        in_display          : &glium::backend::glutin_backend::GlutinFacade,
        in_default_programs : &'a program::DefaultPrograms
    ) -> DefaultModels<'a> {
        let sr_1_2 = 1.0/2.0f32.sqrt();    // for tetrahedron
        let phi = 2.0/(1.0+5.0f32.sqrt()); // for icosahedron

        DefaultModels {
            // ==============================
            // triangle
            // ==============================
            _triangle : Model::new(
                in_display,
                &vec! [
                    Vertex::new([-1.0, -1.0, 0.0], [0.0, 0.0, 1.0]),
                    Vertex::new([-1.0,  1.0, 0.0], [0.0, 0.0, 1.0]),
                    Vertex::new([ 1.0,  0.0, 0.0], [0.0, 0.0, 1.0]),
                ],
                &glium::index::PrimitiveType::TriangleStrip,
                &vec![0, 1, 2u16],
                in_default_programs.polyhedron(),
            ),

            // ==============================
            // square
            // ==============================
            _square : Model::new(
                in_display,
                &vec! [
                    Vertex::new([-1.0, -1.0, 0.0], [0.0, 0.0, 1.0]),
                    Vertex::new([ 1.0, -1.0, 0.0], [0.0, 0.0, 1.0]),
                    Vertex::new([-1.0,  1.0, 0.0], [0.0, 0.0, 1.0]),
                    Vertex::new([ 1.0,  1.0, 0.0], [0.0, 0.0, 1.0]),
                ],
                &glium::index::PrimitiveType::TriangleStrip,
                &vec![0, 2, 1, 3u16],
                in_default_programs.polyhedron(),
            ),

            // ==============================
            // tetrahedron
            // ==============================
            _tetrahedron : Model::new(
                in_display,
                &vec![
                    Vertex::new([-1.0,  0.0, -sr_1_2],[-1.0,  0.0, -sr_1_2]),
                    Vertex::new([ 1.0,  0.0, -sr_1_2],[ 1.0,  0.0, -sr_1_2]),
                    Vertex::new([ 0.0, -1.0,  sr_1_2],[ 0.0, -1.0,  sr_1_2]),
                    Vertex::new([ 0.0,  1.0,  sr_1_2],[ 0.0,  1.0,  sr_1_2]),
                ],
                &glium::index::PrimitiveType::TriangleStrip,
                &vec![0, 1, 3, 2, 0, 1u16],
                in_default_programs.polyhedron(),
            ),

            // ==============================
            // cube
            // ==============================
            // currently has weird rounded edges because of normal interpolation.
            // Different vertices should be used for different faces at each corner.
            // n.b. uses TrianglesList not TriangleStrip, because triangle strips don't do corners.
            _cube : Model::new(
                in_display,
                &vec![
                    Vertex::new([-1.0, -1.0, -1.0],[-1.0, -1.0, -1.0]),
                    Vertex::new([ 1.0, -1.0, -1.0],[ 1.0, -1.0, -1.0]),
                    Vertex::new([-1.0,  1.0, -1.0],[-1.0,  1.0, -1.0]),
                    Vertex::new([ 1.0,  1.0, -1.0],[ 1.0,  1.0, -1.0]),
                    Vertex::new([-1.0, -1.0,  1.0],[-1.0, -1.0,  1.0]),
                    Vertex::new([ 1.0, -1.0,  1.0],[ 1.0, -1.0,  1.0]),
                    Vertex::new([-1.0,  1.0,  1.0],[-1.0,  1.0,  1.0]),
                    Vertex::new([ 1.0,  1.0,  1.0],[ 1.0,  1.0,  1.0])
                ],
                &glium::index::PrimitiveType::TrianglesList,
                &vec![
                    0, 2, 1, 3, 1, 2,   // the -z face
                    2, 6, 3, 7, 3, 6,   // the  y face
                    4, 5, 6, 7, 6, 5,   // the  z face
                    0, 1, 4, 5, 4, 1,   // the -y face
                    1, 3, 5, 7, 5, 3,   // the  x face
                    0, 4, 2, 6, 2, 4u16 // the -x face
                ],
                in_default_programs.polyhedron(),
            ),

            // ==============================
            // icosahedron
            // ==============================
            _icosahedron : Model::new(
                in_display,
                &vec![
                    Vertex::new([ 0.0,  1.0,  phi],[ 0.0,  1.0,  phi]),
                    Vertex::new([ 0.0, -1.0,  phi],[ 0.0, -1.0,  phi]),
                    Vertex::new([ 0.0,  1.0, -phi],[ 0.0,  1.0, -phi]),
                    Vertex::new([ 0.0, -1.0, -phi],[ 0.0, -1.0, -phi]),
                    Vertex::new([ phi,  0.0,  1.0],[ phi,  0.0,  1.0]),
                    Vertex::new([ phi,  0.0, -1.0],[ phi,  0.0, -1.0]),
                    Vertex::new([-phi,  0.0,  1.0],[-phi,  0.0,  1.0]),
                    Vertex::new([-phi,  0.0, -1.0],[-phi,  0.0, -1.0]),
                    Vertex::new([ 1.0,  phi,  0.0],[ 1.0,  phi,  0.0]),
                    Vertex::new([-1.0,  phi,  0.0],[-1.0,  phi,  0.0]),
                    Vertex::new([ 1.0, -phi,  0.0],[ 1.0, -phi,  0.0]),
                    Vertex::new([-1.0, -phi,  0.0],[-1.0, -phi,  0.0]),
                ],
                &glium::index::PrimitiveType::TrianglesList,
                &vec![
                    0, 8, 2,
                    0, 2, 9,
                    1, 3, 10,
                    1, 11, 3,
                    4, 0, 6,
                    4, 6, 1,
                    5, 7, 2,
                    5, 3, 7,
                    8, 4, 10,
                    8, 10, 5,
                    9, 11, 6,
                    9, 7, 11,
                    0, 4, 8,
                    0, 9, 6,
                    1, 10, 4,
                    1, 6, 11,
                    2, 8, 5,
                    2, 7, 9,
                    3, 5, 10,
                    3, 11, 7u16
                ],
                in_default_programs.polyhedron(),
            ),

            // ==============================
            // sphere
            // ==============================
            _sphere : Model::new(
                in_display,
                &vec! [
                    Vertex::new([-1.0, -1.0, 0.0], [-1.0, -1.0, 0.0]),
                    Vertex::new([ 1.0, -1.0, 0.0], [ 1.0, -1.0, 0.0]),
                    Vertex::new([-1.0,  1.0, 0.0], [-1.0,  1.0, 0.0]),
                    Vertex::new([ 1.0,  1.0, 0.0], [ 1.0,  1.0, 0.0]),
                ],
                &glium::index::PrimitiveType::TriangleStrip,
                &vec![0, 2, 1, 3u16],
                in_default_programs.sphere(),
            ),
        }
    }

    #[allow(dead_code)]
    pub fn triangle(&self) -> &Model {&self._triangle}
    #[allow(dead_code)]
    pub fn square(&self) -> &Model {&self._square}
    #[allow(dead_code)]
    pub fn tetrahedron(&self) -> &Model {&self._tetrahedron}
    #[allow(dead_code)]
    pub fn cube(&self) -> &Model {&self._cube}
    #[allow(dead_code)]
    pub fn icosahedron(&self) -> &Model {&self._icosahedron}
    #[allow(dead_code)]
    pub fn sphere(&self) -> &Model {&self._sphere}
}

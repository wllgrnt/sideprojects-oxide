extern crate glium;

use vertex::Vertex;

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

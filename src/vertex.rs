// ============================================================
// Vertex
// ============================================================
#[derive(Copy, Clone)]
pub struct Vertex {
    _position : [f32;4],
    _normal   : [f32;4],
}

impl Vertex {
    pub fn new(in_position : &[f32; 3], in_normal : &[f32;3]) -> Vertex {
        Vertex {
            _position : [in_position[0],in_position[1],in_position[2],1.0],
            _normal   : [in_normal[0],in_normal[1],in_normal[2],0.0]
        }
    }
}

implement_vertex!(Vertex, _position, _normal);

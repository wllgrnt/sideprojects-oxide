#[macro_use]
extern crate glium;

// ==============================
// Vec4 and Mat4
// ==============================
// I'm adding custom vectors and matrices so we can add a proper linear algebra library more easily later - MJ
// It will also be useful when we come to add unit normals (atm these are just wrappers) - MJ
#[derive(Copy, Clone)]
struct Vec4 {
	position : [f32;4],
}

impl Vec4 {
	fn new(in_contents : [f32; 3]) -> Vec4 {
		Vec4 {
			position: [in_contents[0],in_contents[1],in_contents[2],1.0]
		}
	}
}

#[derive(Copy, Clone)]
struct Mat4 {
	_contents : [[f32;4];4]
}

impl Mat4 {
	fn new(in_contents : [[f32;4];4]) -> Mat4 {
		Mat4 {
			_contents: in_contents,
		}
	}
}


// ==============================
// Mesh
// ==============================
/// The mesh of a single object (a triangle, a sphere, a goove...)
struct Mesh {
	_vertices : Vec<Vec4>,
	_indices : Vec<u16>,
	// these buffers should be moved to whatever class holds all the things when that class is made
	_vertex_buffer : glium::VertexBuffer<Vec4>,
	_index_buffer  : glium::index::IndexBuffer<u16>,
}

impl Mesh {
	fn new (
		in_display  : &glium::backend::glutin_backend::GlutinFacade,
		in_vertices : Vec<Vec4>,
		in_indices  : Vec<u16>,
	) -> Mesh {
		Mesh {
			_vertex_buffer : glium::VertexBuffer::new(in_display, &in_vertices).unwrap(),
			_index_buffer  : glium::index::IndexBuffer::new (
				in_display,
				glium::index::PrimitiveType::TriangleStrip,
				&in_indices,
			).unwrap(),
			_vertices : in_vertices,
			_indices  : in_indices,
		}
	}
	
	fn vertex_buffer(&self) -> &glium::VertexBuffer<Vec4> {&self._vertex_buffer}
	fn index_buffer(&self) -> &glium::index::IndexBuffer<u16> {&self._index_buffer}
}

// ==============================
// Main Program
// ==============================
/// Furnace - draw a triangle!
fn main() {
    use glium::{DisplayBuild, Surface};
    let display : glium::backend::glutin_backend::GlutinFacade = glium::glutin::WindowBuilder::new()
        .with_title("Furnace: Molecular Visualisation".to_string())
        .build_glium().unwrap();

    implement_vertex!(Vec4, position);

    // The positions of each vertex of the triangle
    let vertex1 = Vec4::new([-0.5, -0.5 , 0.0]);
    let vertex2 = Vec4::new([ 0.0,  0.5 , 0.0]);
    let vertex3 = Vec4::new([ 0.5, -0.25, 0.0]);
		let triangle = Mesh::new(&display, vec![vertex1, vertex2, vertex3], vec![0u16, 1u16, 2u16]);

    // The positions of each vertex of the square
    let vertex4 = Vec4::new([-0.25, -0.25, 0.0]);
    let vertex5 = Vec4::new([ 0.25, -0.25, 0.0]);
    let vertex6 = Vec4::new([-0.25,  0.25, 0.0]);
    let vertex7 = Vec4::new([ 0.25,  0.25, 0.0]);
		let triangle2 = Mesh::new(&display, vec![vertex4, vertex5, vertex6, vertex7], vec![0u16, 1u16, 2u16, 3u16]);
    
    // Vertex shader in OpenGL v140 (written in GLSL) 
    let vertex_shader_src = r#"
        #version 140

        in vec4 position;

        void main() {
            gl_Position = position;
        }
    "#;

    // Fragment/Pixel shader in OpenGL v140 (written in GLSL) 
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.847, 0.359375, 0.007812, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.93, 0.91, 0.835, 1.0);
        target.draw(
					triangle2.vertex_buffer(),
					triangle2.index_buffer(),
					&program,
					&glium::uniforms::EmptyUniforms,
          &Default::default()
				).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

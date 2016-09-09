#[macro_use]
extern crate glium;

// ==============================
// Vertex
// ==============================
#[derive(Copy, Clone)]
struct Vertex {
	position : [f32;4],
}

impl Vertex {
	fn new(in_vertex : [f32; 3]) -> Vertex {
		Vertex {
			position: [in_vertex[0],in_vertex[1],in_vertex[2],1.0]
		}
	}
}

// ==============================
// Matrix
// ==============================
// NB: OpenGL treats vectors as row vectors, so matrices must be transposed and multiplication reversed.
// TODO: we really need proper linear algebra, or matrix stuff will be super slow.
/// A 4x4 matrix for holding transformations.
struct Matrix {
	_contents : [[f32; 4]; 4]
}

impl Matrix {
	fn new(in_matrix : [[f32; 4]; 4]) -> Matrix {
		Matrix {
			_contents: in_matrix
		}
	}
}

// ==============================
// Mesh
// ==============================
/// The mesh of a single object (a triangle, a sphere, a goove...)
struct Mesh {
	/// The vertices of the triangles out of which the mesh is made
	_vertices      : Vec<Vertex>,
	/// The order in which the vertices should be drawn.
	_indices       : Vec<u16>,
	_vertex_buffer : glium::VertexBuffer<Vertex>,
	_index_buffer  : glium::index::IndexBuffer<u16>,
}

impl Mesh {
	fn new (
		in_display  : &glium::backend::glutin_backend::GlutinFacade,
		in_vertices : &Vec<Vertex>,
		in_indices  : &Vec<u16>,
	) -> Mesh {
		Mesh {
			_vertices      : in_vertices.to_owned(),
			_indices       : in_indices.to_owned(),
			_vertex_buffer : glium::VertexBuffer::new(in_display, in_vertices).unwrap(),
			_index_buffer  : glium::index::IndexBuffer::new (
				in_display,
				glium::index::PrimitiveType::TriangleStrip,
				in_indices,
			).unwrap(),
		}
	}
	
	fn vertex_buffer(&self) -> &glium::VertexBuffer<Vertex> {&self._vertex_buffer}
	fn index_buffer(&self) -> &glium::index::IndexBuffer<u16> {&self._index_buffer}
}


// ==============================
// Atom
// ==============================
/// The atom, the fundamental unit of a molecular viewer.
struct Atom<'a> {
	_mesh        : &'a Mesh,
	_position    : Vertex,
	_body_matrix : Matrix,
}

impl<'a> Atom<'a> {
	fn new (
		in_mesh     : &'a Mesh,
		in_position : &Vertex,
	) -> Atom<'a> {
		Atom {
			_mesh : in_mesh,
			_position : in_position.to_owned(),
			_body_matrix : Matrix::new([[1.0,0.0,0.0,0.0],[0.0,1.0,0.0,0.0],[0.0,0.0,1.0,0.0],[0.0,0.0,0.0,1.0]]),
		}
	}
	
	fn mesh(&self) -> &Mesh {&self._mesh}
}

// ==============================
// Molecule
// ==============================
// Will likely be the top level struct, unless we need something which has an OpenGL thing + this
/// The molecule. May also be a cluster, crystal motif,...
struct Molecule<'a> {
	_atoms : Vec<Atom<'a>>,
}

impl<'a> Molecule<'a> {
	fn new() -> Molecule<'a> {Molecule{_atoms : Vec::new()}}
	
	fn add_atom(
		&mut self,
		in_mesh     : &'a Mesh,
		in_position : &Vertex
	) {self._atoms.push(Atom::new(in_mesh, in_position))}
	
	fn atoms(&self) -> &Vec<Atom> {&self._atoms}
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

	implement_vertex!(Vertex, position);

	// The positions of each vertex of the triangle
	let vertex1 = Vertex::new([-0.5 , -0.5, 0.0]);
	let vertex2 = Vertex::new([-0.5 ,  0.5, 0.0]);
	let vertex3 = Vertex::new([-0.75,  0.0, 0.0]);
	let triangle = Mesh::new(&display, &vec![vertex1, vertex2, vertex3], &vec![0, 1, 2u16]);

	// The positions of each vertex of the square
	let vertex4 = Vertex::new([-0.25, -0.25, 0.0]);
	let vertex5 = Vertex::new([ 0.25, -0.25, 0.0]);
	let vertex6 = Vertex::new([-0.25,  0.25, 0.0]);
	let vertex7 = Vertex::new([ 0.25,  0.25, 0.0]);
	let square = Mesh::new(&display, &vec![vertex4, vertex5, vertex6, vertex7], &vec![0, 1, 2, 3u16]);
	
	let position1 = Vertex::new([0.0, 0.0, 0.0]);
	
	let position2 = Vertex::new([0.0, 0.0, 0.0]);
	
	let mut molecule = Molecule::new();
	molecule.add_atom(&triangle, &position1);
	molecule.add_atom(&square, &position2);
	
	// Vertex shader in OpenGL v140 (written in GLSL) 
	let vertex_shader_src = r#"
	#version 140
	
	uniform mat4 matrix;
	
	in vec4 position;

	void main() {
		gl_Position = position*matrix;
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
	
	let view_matrix = [
		[1.0, 0.0, 0.0, 0.0],
		[0.0, 1.0, 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0f32]
	];
	
	loop {
		let uniforms = uniform!{matrix: view_matrix};
		let mut target = display.draw();
		target.clear_color(0.93, 0.91, 0.835, 1.0);
		for atom in molecule.atoms() {
			target.draw(
				atom.mesh().vertex_buffer(),
				atom.mesh().index_buffer(),
				&program,
				&uniforms,
				&Default::default()
			).unwrap();
		}
		target.finish().unwrap();

		for ev in display.poll_events() {
			match ev {
				glium::glutin::Event::Closed => return,
				_ => ()
			}
		}
	}
}

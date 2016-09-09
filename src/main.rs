#[macro_use]
extern crate glium;

use std::ops::Mul;

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
/// A 4x4 matrix for holding transformations.
#[derive(Copy, Clone)]
struct Matrix {
	_contents : [[f32; 4]; 4]
}

impl Matrix {
	fn new(in_matrix : [[f32; 4]; 4]) -> Matrix {
		Matrix {
			_contents: in_matrix
		}
	}
	
	fn contents(&self) -> &[[f32;4];4] {&self._contents}
}

// Matrix multiplication. TODO: use a linear algebra library.
impl Mul for Matrix {
	type Output = Matrix;
	
	fn mul (self, in_other : Matrix) -> Matrix {
		let a : &[[f32;4];4] = &self._contents;
		let b : &[[f32;4];4] = &in_other._contents;
		Matrix::new([[
			a[0][0]*b[0][0]+a[0][1]*b[1][0]+a[0][2]*b[2][0]+a[0][3]*b[3][0],
			a[0][0]*b[0][1]+a[0][1]*b[1][1]+a[0][2]*b[2][1]+a[0][3]*b[3][1],
			a[0][0]*b[0][2]+a[0][1]*b[1][2]+a[0][2]*b[2][2]+a[0][3]*b[3][2],
			a[0][0]*b[0][3]+a[0][1]*b[1][3]+a[0][2]*b[2][3]+a[0][3]*b[3][3]
		], [
			a[1][0]*b[0][0]+a[1][1]*b[1][0]+a[1][2]*b[2][0]+a[1][3]*b[3][0],
			a[1][0]*b[0][1]+a[1][1]*b[1][1]+a[1][2]*b[2][1]+a[1][3]*b[3][1],
			a[1][0]*b[0][2]+a[1][1]*b[1][2]+a[1][2]*b[2][2]+a[1][3]*b[3][2],
			a[1][0]*b[0][3]+a[1][1]*b[1][3]+a[1][2]*b[2][3]+a[1][3]*b[3][3]
		], [
			a[2][0]*b[0][0]+a[2][1]*b[1][0]+a[2][2]*b[2][0]+a[2][3]*b[3][0],
			a[2][0]*b[0][1]+a[2][1]*b[1][1]+a[2][2]*b[2][1]+a[2][3]*b[3][1],
			a[2][0]*b[0][2]+a[2][1]*b[1][2]+a[2][2]*b[2][2]+a[2][3]*b[3][2],
			a[2][0]*b[0][3]+a[2][1]*b[1][3]+a[2][2]*b[2][3]+a[2][3]*b[3][3]
		], [
			a[3][0]*b[0][0]+a[3][1]*b[1][0]+a[3][2]*b[2][0]+a[3][3]*b[3][0],
			a[3][0]*b[0][1]+a[3][1]*b[1][1]+a[3][2]*b[2][1]+a[3][3]*b[3][1],
			a[3][0]*b[0][2]+a[3][1]*b[1][2]+a[3][2]*b[2][2]+a[3][3]*b[3][2],
			a[3][0]*b[0][3]+a[3][1]*b[1][3]+a[3][2]*b[2][3]+a[3][3]*b[3][3]
		]])
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
	_position    : [f32;3],
	_size        : f32,
	_body_matrix : Matrix,
}

impl<'a> Atom<'a> {
	fn new (
		in_mesh     : &'a Mesh,
		in_position : &[f32;3],
		in_size     : &f32
	) -> Atom<'a> {
		Atom {
			_mesh        : in_mesh,
			_position    : in_position.to_owned(),
			_size        : in_size.to_owned(),
			_body_matrix : Matrix::new([
				[*in_size, 0.0     , 0.0     ,in_position[0]],
				[0.0     , *in_size, 0.0     ,in_position[1]],
				[0.0     , 0.0     , *in_size,in_position[2]],
				[0.0     , 0.0     , 0.0     ,1.0]
			]),
		}
	}
	
	fn mesh(&self) -> &Mesh {&self._mesh}
	fn body_matrix(&self) -> &Matrix {&self._body_matrix}
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
		in_position : &[f32;3],
		in_size     : &f32,
	) {self._atoms.push(Atom::new(in_mesh, in_position, in_size))}
	
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
	let vertex1 = Vertex::new([-1.0, -1.0, 0.0]);
	let vertex2 = Vertex::new([-1.0,  1.0, 0.0]);
	let vertex3 = Vertex::new([ 1.0,  0.0, 0.0]);
	let triangle = Mesh::new(&display, &vec![vertex1, vertex2, vertex3], &vec![0, 1, 2u16]);

	// The positions of each vertex of the square
	let vertex4 = Vertex::new([-1.0, -1.0, 0.0]);
	let vertex5 = Vertex::new([ 1.0, -1.0, 0.0]);
	let vertex6 = Vertex::new([-1.0,  1.0, 0.0]);
	let vertex7 = Vertex::new([ 1.0,  1.0, 0.0]);
	let square = Mesh::new(&display, &vec![vertex4, vertex5, vertex6, vertex7], &vec![0, 1, 2, 3u16]);
	
	//let position1 = [0.0, 0.0, 0.0];
	
	//let position2 = [0.5, 0.0, 0.0];
	
	let mut molecule = Molecule::new();
	molecule.add_atom(&triangle, &[ 0.0,  0.0, 0.0], &0.2);
	molecule.add_atom(&triangle, &[ 0.5,  0.5, 0.0], &0.2);
	molecule.add_atom(&triangle, &[ 0.5, -0.5, 0.0], &0.2);
	molecule.add_atom(&triangle, &[-0.5,  0.5, 0.0], &0.2);
	molecule.add_atom(&triangle, &[-0.5, -0.5, 0.0], &0.2);
	molecule.add_atom(&square, &[ 0.5,  0.0, 0.0], &0.2);
	molecule.add_atom(&square, &[-0.5,  0.0, 0.0], &0.2);
	molecule.add_atom(&square, &[ 0.0,  0.5, 0.0], &0.2);
	molecule.add_atom(&square, &[ 0.0, -0.5, 0.0], &0.2);
	
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
	
	let camera_matrix = Matrix::new([
		[1.0, 0.0, 0.0, 0.0],
		[0.0, 1.0, 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0]
	]);
	
	let perspective_matrix = Matrix::new([
		[1.0, 0.0, 0.0, 0.0],
		[0.0, 1.0, 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0]
	]);
	
	let view_matrix = perspective_matrix*camera_matrix;
	
	loop {
		let mut target = display.draw();
		target.clear_color(0.93, 0.91, 0.835, 1.0);
		for atom in molecule.atoms() {
			let matrix = *atom.body_matrix() * view_matrix;
			let uniforms = uniform!{matrix: matrix.contents().to_owned()};
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

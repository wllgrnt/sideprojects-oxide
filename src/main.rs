#[macro_use]
extern crate glium;

use std::f32;      //pi
use std::ops::Mul; // multiplication overload


mod fxaa;

// ============================================================
// Vertex
// ============================================================
#[derive(Copy, Clone)]
struct Vertex {
    _position : [f32;4],
    _normal   : [f32;4],
}

impl Vertex {
    fn new(in_position : [f32; 3], in_normal : [f32;3]) -> Vertex {
        Vertex {
            _position : [in_position[0],in_position[1],in_position[2],1.0],
            _normal   : [in_normal[0],in_normal[1],in_normal[2],0.0]
        }
    }
}

// ============================================================
// Matrix
// ============================================================
// NB: OpenGL (maybe) treats vectors as row vectors, so matrices should be transposed and multiplication reversed?
/// A 4x4 matrix for holding transformations.
#[derive(Copy, Clone)]
struct Matrix {
    _contents : [[f32; 4]; 4]
}

impl Matrix {
    fn new(in_contents : [[f32; 4]; 4]) -> Matrix {
        Matrix {
            _contents: in_contents
        }
    }

    fn contents(&self) -> &[[f32;4];4] {&self._contents}
}

// Matrix multiplication. TODO: use a linear algebra library.
impl Mul<Matrix> for Matrix {
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

impl Mul<[f32;4]> for Matrix {
    type Output = [f32;4];

    fn mul (self, in_other : [f32;4]) -> [f32;4] {
        let a : &[[f32;4];4] = &self._contents;
        let b : &[f32;4] = &in_other;
        [
            a[0][0]*b[0]+a[0][1]*b[1]+a[0][2]*b[2]+a[0][3]*b[3],
            a[1][0]*b[0]+a[1][1]*b[1]+a[1][2]*b[2]+a[1][3]*b[3],
            a[2][0]*b[0]+a[2][1]*b[1]+a[2][2]*b[2]+a[2][3]*b[3],
            a[3][0]*b[0]+a[3][1]*b[1]+a[3][2]*b[2]+a[3][3]*b[3]
        ]
    }
}

// ============================================================
// Mesh
// ============================================================
/// The mesh of a single object (a triangle, a sphere, a goove...)
struct Mesh<'a> {
    /// The vertices of the triangles out of which the mesh is made
    _vertices        : Vec<Vertex>,
    /// The order in which the vertices should be drawn.
    _index_type      : glium::index::PrimitiveType,
    _indices         : Vec<u16>,
    _program         : &'a glium::Program,
    _vertex_buffer   : glium::VertexBuffer<Vertex>,
    _index_buffer    : glium::index::IndexBuffer<u16>,
}

impl<'a> Mesh<'a> {
    fn new (
        in_display    : &glium::backend::glutin_backend::GlutinFacade,
        in_vertices   : &Vec<Vertex>,
        in_index_type : &glium::index::PrimitiveType,
        in_indices    : &Vec<u16>,
        in_program    : &'a glium::Program,
    ) -> Mesh<'a> {
        Mesh {
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

    fn vertex_buffer(&self) -> &glium::VertexBuffer<Vertex> {&self._vertex_buffer}
    fn index_buffer(&self) -> &glium::index::IndexBuffer<u16> {&self._index_buffer}
    fn program(&self) -> &glium::Program {&self._program}
}


// ============================================================
// Species
// ============================================================
struct Species<'a> {
    _mesh   : &'a Mesh<'a>,
    _size   : f32,
    _colour : [f32;3],
}

impl<'a> Species<'a> {
    fn new (
        in_mesh   : &'a Mesh,
        in_size   : &f32,
        in_colour : &[f32;3],
    ) -> Species<'a> {
        Species {
            _mesh   : in_mesh,
            _size   : in_size.to_owned(),
            _colour : in_colour.to_owned()
        }
    }

    fn mesh(&self) -> &Mesh {&self._mesh}
    fn size(&self) -> &f32  {&self._size}
    fn colour(&self) -> &[f32;3] {&self._colour}
}

// ============================================================
// Atom
// ============================================================
/// The atom, the fundamental unit of a molecular viewer.
struct Atom<'a> {
    _species      : &'a Species<'a>,
    _position     : [f32;3],
    _model_matrix : Matrix,
}

impl<'a> Atom<'a> {
    fn new (
        in_species  : &'a Species,
        in_position : &[f32;3],
    ) -> Atom<'a> {
        Atom {
            _species      : in_species,
            _position     : in_position.to_owned(),
            _model_matrix : Matrix::new([
                [*in_species.size(), 0.0               , 0.0               , in_position[0]],
                [0.0               , *in_species.size(), 0.0               , in_position[1]],
                [0.0               , 0.0               , *in_species.size(), in_position[2]],
                [0.0               , 0.0               , 0.0               , 1.0           ]
            ]),
        }
    }

    fn species(&self) -> &Species<'a> {&self._species}
    fn model_matrix(&self) -> &Matrix {&self._model_matrix}

    fn rotate_against_camera(&mut self, in_camera : &Camera) {

        let translation_and_scaling_matrix = Matrix::new ([
                [*self._species.size(), 0.0, 0.0, self._position[0]],
                [0.0, *self._species.size(), 0.0, self._position[1]],
                [0.0, 0.0, *self._species.size(), self._position[2]],
                [0.0, 0.0, 0.0                  , 1.0              ]
        ]);

        let orbital_matrix = Matrix::new ([
            [ *in_camera.cos_theta(), 0.0, *in_camera.sin_theta(), 0.0],
            [ 0.0                   , 1.0, 0.0                   , 0.0],
            [-*in_camera.sin_theta(), 0.0, *in_camera.cos_theta(), 0.0],
            [ 0.0                   , 0.0, 0.0                   , 1.0]
        ]);

        let azimuthal_matrix = Matrix::new ([
            [1.0,  0.0                 , 0.0                 , 0.0],
            [0.0,  *in_camera.cos_phi(), *in_camera.sin_phi(), 0.0],
            [0.0, -*in_camera.sin_phi(), *in_camera.cos_phi(), 0.0],
            [0.0,  0.0                 , 0.0                 , 1.0]
        ]);

	let spin_matrix = Matrix::new ([
	    [ *in_camera.cos_psi(), *in_camera.sin_psi(), 0.0, 0.0],
	    [-*in_camera.sin_psi(), *in_camera.cos_psi(), 0.0, 0.0],
	    [ 0.0                 , 0.0                 , 1.0, 0.0],
	    [ 0.0                 , 0.0                 , 0.0, 1.0]
	]);

        self._model_matrix = translation_and_scaling_matrix
	                   * orbital_matrix
			   * azimuthal_matrix
			   * spin_matrix;
    }
}

// ============================================================
// Molecule
// ============================================================
// Will likely be the top level struct, unless we need something which has an OpenGL thing + this
/// The molecule. May also be a cluster, crystal motif,...
struct Molecule<'a> {
    _atoms : Vec<Atom<'a>>,
}

impl<'a> Molecule<'a> {
    fn new() -> Molecule<'a> {Molecule{_atoms : Vec::new()}}

    fn add_atom(
        &mut self,
        in_species  : &'a Species,
        in_position : &[f32;3],
    ) {self._atoms.push(Atom::new(in_species, in_position))}

    fn atoms(&self) -> &Vec<Atom> {&self._atoms}

    fn rotate_atoms_against_camera(&mut self, in_camera : &Camera) {
        for atom in &mut self._atoms {
            atom.rotate_against_camera(in_camera);
        }
    }
}


// ============================================================
// Camera
// ============================================================
struct Camera {
    _focus              : [f32;3],
    _theta_degrees      : u32,
    _cos_theta          : f32,
    _sin_theta          : f32,
    _phi_degrees        : u32,
    _cos_phi            : f32,
    _sin_phi            : f32,
    _psi_degrees        : u32,
    _cos_psi            : f32,
    _sin_psi            : f32,
    _r                  : u32,
    _field_of_view      : f32,
    _near_plane         : f32,
    _far_plane          : f32,
    _screen_size        : [u32;2],
    _view_matrix        : Matrix,
    _perspective_matrix : Matrix,
    _vp_matrix          : Matrix,
}

impl Camera {
    fn new (
        in_display       : &glium::backend::glutin_backend::GlutinFacade,
        in_focus         : &[f32;3],
        in_theta_degrees : &u32,
        in_phi_degrees   : &u32,
        in_psi_degrees   : &u32,
        in_r             : &u32,
        in_field_of_view : &f32,
        in_near_plane    : &f32,
        in_far_plane     : &f32
    ) -> Camera {

        let (w, h) = (*in_display).get_framebuffer_dimensions();

        let mut camera = Camera {
            _focus              : in_focus.to_owned(),
            _theta_degrees      : in_theta_degrees.to_owned(),
            _cos_theta          : Default::default(),
	    _sin_theta          : Default::default(),
	    _phi_degrees        : in_phi_degrees.to_owned(),
	    _cos_phi            : Default::default(),
	    _sin_phi            : Default::default(),
            _psi_degrees        : in_psi_degrees.to_owned(),
	    _cos_psi            : Default::default(),
	    _sin_psi            : Default::default(),
            _r                  : in_r.to_owned(),
            _field_of_view      : in_field_of_view.to_owned(),
            _near_plane         : in_near_plane.to_owned(),
            _far_plane          : in_far_plane.to_owned(),
            _screen_size        : [w, h],
            _view_matrix        : Matrix::new([[0.0;4];4]),   // dummy value
            _perspective_matrix : Matrix::new([[0.0;4];4]),   // dummy value
            _vp_matrix          : Matrix::new([[0.0;4];4]),   // dummy value
        };
        camera.update();
        camera
    }
    
    fn cos_theta(&self) -> &f32 {&self._cos_theta}
    fn sin_theta(&self) -> &f32 {&self._sin_theta}
    fn cos_phi(&self) -> &f32 {&self._cos_phi}
    fn sin_phi(&self) -> &f32 {&self._sin_phi}
    fn cos_psi(&self) -> &f32 {&self._cos_psi}
    fn sin_psi(&self) -> &f32 {&self._sin_psi}
    fn view_matrix(&self) -> &Matrix {&self._view_matrix}
    fn vp_matrix(&self) -> &Matrix {&self._vp_matrix}

    /*fn set_position(&mut self, in_position : [f32;3]) {
        self._position = in_position;
        self.update();
    }*/
    
    fn set_angles(
        &mut self,
        in_theta_degrees : &u32,
        in_phi_degrees   : &u32,
        in_psi_degrees   : &u32,
        in_r             : &u32
    ) {
        self._theta_degrees = in_theta_degrees.to_owned();
        self._phi_degrees = in_phi_degrees.to_owned();
	self._psi_degrees = in_psi_degrees.to_owned();
	self._r = in_r.to_owned();
	self.update();
    }
    
    fn set_screen_size(&mut self, in_x : &u32, in_y : &u32) {
        self._screen_size = [*in_x, *in_y];
    }
    
    fn update(&mut self) {
        
        let mut w = self._screen_size[0] as f32;
        let mut h = self._screen_size[1] as f32;
        if w > h {
            w = w/h;
            h = 1.0;
        } else {
            h = h/w;
            w = 1.0;
        }
        
        let s = 1.0/(self._field_of_view*f32::consts::PI/360.0).tan();
        let n = self._near_plane.to_owned();
        let f = self._far_plane.to_owned();
        self._perspective_matrix = Matrix::new([
            [s/w, 0.0, 0.0        , 0.0          ],
            [0.0, s/h, 0.0        , 0.0          ],
            [0.0, 0.0, (f+n)/(f-n), 2.0*f*n/(n-f)],
            [0.0, 0.0, 1.0        , 0.0          ]
        ]);
        
	// Translate so that the focus is centred.
        let focus_translation_matrix = Matrix::new([
            [1.0, 0.0, 0.0, -self._focus[0]],
            [0.0, 1.0, 0.0, -self._focus[1]],
            [0.0, 0.0, 1.0, -self._focus[2]],
            [0.0, 0.0, 0.0,  1.0           ]
        ]);

        // theta is the orbital angle
	let theta = (self._theta_degrees as f32)*std::f32::consts::PI/180.0;
        self._cos_theta =  theta.cos();
        self._sin_theta =  theta.sin();
        let orbital_matrix = Matrix::new([
            [ self._cos_theta, 0.0, -self._sin_theta, 0.0],
            [ 0.0            , 1.0,  0.0            , 0.0],
            [ self._sin_theta, 0.0,  self._cos_theta, 0.0],
            [ 0.0            , 0.0,  0.0            , 1.0]
        ]);

        // phi is the azimuthal angle
	let phi = (self._phi_degrees as f32)*std::f32::consts::PI/180.0;
        self._cos_phi = phi.cos();
        self._sin_phi = phi.sin();
        let azimuthal_matrix = Matrix::new([
            [1.0, 0.0          ,  0.0          , 0.0],
            [0.0, self._cos_phi, -self._sin_phi, 0.0],
            [0.0, self._sin_phi,  self._cos_phi, 0.0],
            [0.0, 0.0          ,  0.0          , 1.0]
        ]);
        
        // psi is the spin angle
        let psi = (self._psi_degrees as f32)*std::f32::consts::PI/180.0;
	self._cos_psi = psi.cos();
	self._sin_psi = psi.sin();
	let spin_matrix = Matrix::new([
	    [self._cos_psi, -self._sin_psi, 0.0, 0.0],
	    [self._sin_psi,  self._cos_psi, 0.0, 0.0],
	    [0.0          ,  0.0          , 1.0, 0.0],
	    [0.0          ,  0.0          , 0.0, 1.0]
	]);

	// r is the distance of the camera from the focus
	let radius = self._r as f32;
	let zoom_matrix = Matrix::new([
	    [1.0, 0.0, 0.0, 0.0   ],
	    [0.0, 1.0, 0.0, 0.0   ],
	    [0.0, 0.0, 1.0, radius],
	    [0.0, 0.0, 0.0, 1.0   ]
	]);

        self._view_matrix = zoom_matrix
	                  * spin_matrix
	                  * azimuthal_matrix
			  * orbital_matrix
			  * focus_translation_matrix;
        self._vp_matrix = self._perspective_matrix*self._view_matrix;
    }
}


// ============================================================
// Light
// ============================================================

// ============================================================
// Main Program
// ============================================================
/// Furnace - draw a molecule!
fn main() {
    // ==============================
    // Make display
    // ==============================
    use glium::{DisplayBuild, Surface};
    let display : glium::backend::glutin_backend::GlutinFacade = glium::glutin::WindowBuilder::new()
        .with_title("Furnace: Molecular Visualisation".to_string())
        .build_glium().unwrap();

    implement_vertex!(Vertex, _position, _normal);

    // ==============================
    // Dark2
    // ==============================

    let turquoise = [ 27.0/255.0,158.0/255.0,119.0/255.0];
    let orange    = [217.0/255.0, 95.0/255.0,  2.0/255.0];
    let blue      = [117.0/255.0,112.0/255.0,179.0/255.0];
    let pink      = [231.0/255.0, 41.0/255.0,138.0/255.0];
    let green     = [102.0/255.0,166.0/255.0, 30.0/255.0];
    let yellow    = [230.0/255.0,171.0/255.0,  2.0/255.0];
    let brown     = [166.0/255.0,118.0/255.0, 29.0/255.0];
    let grey      = [102.0/255.0,102.0/255.0,102.0/255.0];

    // ==============================
    // Make shaders
    // ==============================
    // ====================
    // Polyhedron shaders
    // ====================
    // Vertex shader in OpenGL v140 (written in GLSL)
    let vertex_shader_polyhedron : &'static str = r#"
        #version 140

        uniform mat4 mv_matrix;
        uniform mat4 mvp_matrix;
        uniform vec4 light_position;

        in vec4 _position;
        in vec4 _normal;

        out vec3 fragment_normal;
        out vec3 fragment_light_vector;

        void main() {
            vec4 position = _position*mv_matrix;
            vec4 normal = normalize(_normal*mv_matrix);
            vec4 light_vector = light_position-position;

            fragment_normal = vec3(normal[0],normal[1],normal[2]);
            fragment_light_vector = vec3(light_vector[0],light_vector[1],light_vector[2]);

            gl_Position = _position*mvp_matrix;
        }
    "#;

    // Fragment/Pixel shader in OpenGL v140 (written in GLSL)
    let fragment_shader_polyhedron : &'static str = r#"
        #version 140

        uniform vec3 colour;

        in vec3 fragment_normal;
        in vec3 fragment_light_vector;

        out vec4 color;

        void main() {
            float normal_squared = dot(fragment_normal,fragment_normal);
            float light_distance_squared = dot(fragment_light_vector,fragment_light_vector);
            float cos_light_angle = clamp (
                dot(fragment_normal,fragment_light_vector)
                    * inversesqrt(light_distance_squared*normal_squared),
                0,
                1
            );
            vec3 colour3 = colour*(cos_light_angle/light_distance_squared+0.2);
            color = vec4((colour3), 1.0);
        }
    "#;

    let program_polyhedron : glium::Program = glium::Program::from_source(
        &display,
        vertex_shader_polyhedron,
        fragment_shader_polyhedron,
        None
    ).unwrap();

    // ====================
    // Sphere shaders
    // ====================
    // Vertex shader in OpenGL v140 (written in GLSL)
    let vertex_shader_sphere : &'static str = r#"
        #version 140

        uniform mat4 mv_matrix;
        uniform mat4 mvp_matrix;
        uniform vec4 light_position;

        in vec4 _position;
        in vec4 _normal;
        
        out vec2 fragment_xy;
        out vec3 fragment_light_vector;

        void main() {
            vec4 position = _position*mv_matrix;
            vec4 light_vector = light_position-position;
            
            fragment_xy = vec2(_normal[0],_normal[1]);
            fragment_light_vector = vec3(light_vector[0],light_vector[1],light_vector[2]);

            gl_Position = _position*mvp_matrix;
        }
    "#;

    // Fragment/Pixel shader in OpenGL v140 (written in GLSL)
    let fragment_shader_sphere : &'static str = r#"
        #version 140

        uniform vec3 colour;
        uniform float size;
        
        in vec2 fragment_xy;
        in vec3 fragment_light_vector;

        out vec4 color;

        void main() {
            float xy_squared = dot(fragment_xy,fragment_xy);
            if (xy_squared > 1)
                discard;
            vec3 normal = vec3(fragment_xy[0],fragment_xy[1],-sqrt(1-xy_squared));
            vec3 light_vector = vec3 (
                fragment_light_vector[0],
                fragment_light_vector[1],
                fragment_light_vector[2]-size*normal[2]
            );
            float light_distance_squared = dot(light_vector,light_vector);
            float cos_light_angle = clamp (
                dot(normal,light_vector) * inversesqrt(light_distance_squared),
                0,
                1
            );
            vec3 colour3 = colour*(cos_light_angle/light_distance_squared+0.2);
            color = vec4(colour3, 1.0);
        }
    "#;

    let program_sphere : glium::Program = glium::Program::from_source(
        &display,
        vertex_shader_sphere,
        fragment_shader_sphere,
        None
    ).unwrap();


    // ==============================
    // Make meshes
    // ==============================
    let sr_1_2 = 1.0/2.0f32.sqrt();

    // The positions of each vertex of the triangle
    let triangle_vertex0 = Vertex::new([-1.0, -1.0, 0.0], [0.0, 0.0, 1.0]);
    let triangle_vertex1 = Vertex::new([-1.0,  1.0, 0.0], [0.0, 0.0, 1.0]);
    let triangle_vertex2 = Vertex::new([ 1.0,  0.0, 0.0], [0.0, 0.0, 1.0]);
    let triangle = Mesh::new(
        &display,
        &vec![triangle_vertex0, triangle_vertex1, triangle_vertex2],
        &glium::index::PrimitiveType::TriangleStrip,
        &vec![0, 1, 2u16],
        &program_polyhedron,
    );

    // The positions of each vertex of the square
    let square_vertex0 = Vertex::new([-1.0, -1.0, 0.0], [0.0, 0.0, 1.0]);
    let square_vertex1 = Vertex::new([ 1.0, -1.0, 0.0], [0.0, 0.0, 1.0]);
    let square_vertex2 = Vertex::new([-1.0,  1.0, 0.0], [0.0, 0.0, 1.0]);
    let square_vertex3 = Vertex::new([ 1.0,  1.0, 0.0], [0.0, 0.0, 1.0]);
    let square = Mesh::new(
        &display,
        &vec![square_vertex0, square_vertex1, square_vertex2, square_vertex3],
        &glium::index::PrimitiveType::TriangleStrip,
        &vec![0, 2, 1, 3u16],
        &program_polyhedron,
    );

    let tetrahedron = Mesh::new(
        &display,
        &vec![
            Vertex::new([-1.0,  0.0, -sr_1_2],[-1.0,  0.0, -sr_1_2]),
            Vertex::new([ 1.0,  0.0, -sr_1_2],[ 1.0,  0.0, -sr_1_2]),
            Vertex::new([ 0.0, -1.0,  sr_1_2],[ 0.0, -1.0,  sr_1_2]),
            Vertex::new([ 0.0,  1.0,  sr_1_2],[ 0.0,  1.0,  sr_1_2]),
        ],
        &glium::index::PrimitiveType::TriangleStrip,
        &vec![0, 1, 3, 2, 0, 1u16],
        &program_polyhedron,
    );

    // A cube (will likely get weird rounded edges because of normal interpolation.
    // Different vertices should be used for different faces at each corner. (not needed since atoms are spheres.)
    // n.b. uses TrianglesList not TriangleStrip, because triangle strips don't do corners.
    let cube = Mesh::new(
        &display,
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
        &program_polyhedron,
    );

    // An icosahedron
    let phi = 2.0/(1.0+5.0f32.sqrt());
    let icosahedron = Mesh::new(
        &display,
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
        &program_polyhedron,
    );

    // The positions of each vertex of the sphere
    let sphere_vertex0 = Vertex::new([-1.0, -1.0, 0.0], [-1.0, -1.0, 0.0]);
    let sphere_vertex1 = Vertex::new([ 1.0, -1.0, 0.0], [ 1.0, -1.0, 0.0]);
    let sphere_vertex2 = Vertex::new([-1.0,  1.0, 0.0], [-1.0,  1.0, 0.0]);
    let sphere_vertex3 = Vertex::new([ 1.0,  1.0, 0.0], [ 1.0,  1.0, 0.0]);
    let sphere = Mesh::new(
        &display,
        &vec![sphere_vertex0, sphere_vertex1, sphere_vertex2, sphere_vertex3],
        &glium::index::PrimitiveType::TriangleStrip,
        &vec![0, 2, 1, 3u16],
        &program_sphere,
    );

    // ==============================
    // Make species
    // ==============================
    let carbon = Species::new(&sphere, &0.1, &orange);
    let nickel = Species::new(&sphere, &0.2, &blue);
    let sulphur = Species::new(&sphere, &0.4, &turquoise);

    // ==============================
    // Make molecule
    // ==============================
    let mut molecule = Molecule::new();
    molecule.add_atom(&sulphur, &[ 0.0,  0.0, 0.0]);
    molecule.add_atom(&nickel, &[ 0.5,  0.5,  0.5]);
    molecule.add_atom(&nickel, &[ 0.5, -0.5,  0.5]);
    molecule.add_atom(&nickel, &[-0.5,  0.5,  0.5]);
    molecule.add_atom(&nickel, &[-0.5, -0.5,  0.5]);
    molecule.add_atom(&nickel, &[ 0.5,  0.5, -0.5]);
    molecule.add_atom(&nickel, &[ 0.5, -0.5, -0.5]);
    molecule.add_atom(&nickel, &[-0.5,  0.5, -0.5]);
    molecule.add_atom(&nickel, &[-0.5, -0.5, -0.5]);
    molecule.add_atom(&carbon, &[ 0.5,  0.0,  0.0]);
    molecule.add_atom(&carbon, &[-0.5,  0.0,  0.0]);
    molecule.add_atom(&carbon, &[ 0.0,  0.5,  0.0]);
    molecule.add_atom(&carbon, &[ 0.0, -0.5,  0.0]);
    molecule.add_atom(&carbon, &[ 0.0,  0.0,  0.5]);
    molecule.add_atom(&carbon, &[ 0.0,  0.0, -0.5]);

    // ==============================
    // Make camera
    // ==============================
    // camera focus (the point the camera is pointing at)
    let camera_focus = [0.0,0.0,0.0];
    // camera position
    let camera_theta = 0;
    let camera_phi = 0;
    let camera_psi = 0;
    let camera_r = 2;
    // field of view, in degrees
    let field_of_view = 90.0;
    // near and far clipping planes
    let near_plane = 1.0;
    let far_plane = 10.0;

    let mut camera = Camera::new (
        &display,
        &camera_focus,
	&camera_theta,
	&camera_phi,
	&camera_psi,
	&camera_r,
        &field_of_view,
        &near_plane,
        &far_plane
    );

    // ==============================
    // Run everything
    // ==============================
    let mut i = 0;
    let spin_divide = 10;

    // this probably wants to be somewhere in the loop.
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling : glium::BackfaceCullingMode::CullCounterClockwise,
        .. Default::default()
    };
    
    let light_position = [2.0,0.0,0.0,1.0f32];

    let mut rotating = true;
    let mut fxaa_enabled = true;
    let fxaa = fxaa::FxaaSystem::new(&display);
    loop {
        for line in camera.view_matrix().contents() {
	    println!("{:5.6} {:5.6} {:5.6} {:5.6}",line[0],line[1],line[2],line[3]);
	}
	println!("");
        let angle = i/spin_divide;
        camera.set_angles(&angle, &45, &angle, &2);
        let light_position = *camera.view_matrix() * light_position;

        molecule.rotate_atoms_against_camera(&camera);

        let mut target = display.draw();
        fxaa::draw(&fxaa, &mut target, fxaa_enabled, |target| {
            target.clear_color_and_depth((0.93, 0.91, 0.835, 1.0), 1.0);
            for atom in molecule.atoms() {
                let mv_matrix = *camera.view_matrix() * *atom.model_matrix();
                let mvp_matrix = *camera.vp_matrix() * *atom.model_matrix();
                let uniforms = uniform!{
                mv_matrix      : mv_matrix.contents().to_owned(),
                mvp_matrix     : mvp_matrix.contents().to_owned(),
                colour         : atom.species().colour().to_owned(),
                light_position : light_position,
                size           : *atom.species().size(),
                };
                target.draw(
                    atom.species().mesh().vertex_buffer(),
                    atom.species().mesh().index_buffer(),
                    atom.species().mesh().program(),
                    &uniforms,
                    &params,
                ).unwrap();
            }
        });
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                // ==============================
                // Window is modified
                // ==============================
                glium::glutin::Event::Closed =>
                    return,
                    glium::glutin::Event::KeyboardInput(
                        glium::glutin::ElementState::Pressed,
                        _,
                        Some(glium::glutin::VirtualKeyCode::Space)
                    )
                => {
                    rotating = !rotating;
                    println!("Rotation is now {}", if rotating { "on" } else { "off" });
                },
                
                glium::glutin::Event::Resized(x, y) => {camera.set_screen_size(&x, &y);},
                
                // ==============================
                // Key is pressed
                // ==============================
                glium::glutin::Event::KeyboardInput (
                    glium::glutin::ElementState::Pressed,
                    _,
                    Some(glium::glutin::VirtualKeyCode::Up)
                ) => {
                    fxaa_enabled = !fxaa_enabled;
                    println!("FXAA is now {}", if fxaa_enabled { "on" } else { "off" });
                },
                
                // ==============================
                // Other
                // ==============================
                _ => ()
            }
        }
        if rotating { i +=1 };
    }
}

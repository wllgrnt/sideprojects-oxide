#[macro_use]
extern crate glium;

use std::f32;      //pi

mod fxaa;
mod vertex;
mod matrix;
mod model;
mod program;
mod species;

use glium::{DisplayBuild, Surface};
use matrix::Matrix;
use species::Species;

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
            [*in_camera.cos_theta(), 0.0, -*in_camera.sin_theta(), 0.0],
            [0.0                   , 1.0,  0.0                   , 0.0],
            [*in_camera.sin_theta(), 0.0,  *in_camera.cos_theta(), 0.0],
            [0.0                   , 0.0,  0.0                   , 1.0]
        ]);

        let azimuthal_matrix = Matrix::new ([
            [1.0, 0.0                 ,  0.0                 , 0.0],
            [0.0, *in_camera.cos_phi(), -*in_camera.sin_phi(), 0.0],
            [0.0, *in_camera.sin_phi(),  *in_camera.cos_phi(), 0.0],
            [0.0, 0.0                 ,  0.0                 , 1.0]
        ]);

	let spin_matrix = Matrix::new ([
	    [*in_camera.cos_psi(), -*in_camera.sin_psi(), 0.0, 0.0],
	    [*in_camera.sin_psi(),  *in_camera.cos_psi(), 0.0, 0.0],
	    [0.0                 ,  0.0                 , 1.0, 0.0],
	    [0.0                 ,  0.0                 , 0.0, 1.0]
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
    _theta              : f32,
    _cos_theta          : f32,
    _sin_theta          : f32,
    _phi                : f32,
    _cos_phi            : f32,
    _sin_phi            : f32,
    _psi                : f32,
    _cos_psi            : f32,
    _sin_psi            : f32,
    _r                  : f32,
    _angular_step       : f32,
    _r_step             : f32,
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
        in_display               : &glium::backend::glutin_backend::GlutinFacade,
        in_focus                 : &[f32;3],
        in_theta_degrees         : &f32,
        in_phi_degrees           : &f32,
        in_psi_degrees           : &f32,
        in_r                     : &f32,
        in_field_of_view_degrees : &f32,
        in_near_plane            : &f32,
        in_far_plane             : &f32
    ) -> Camera {

        let (w, h) = (*in_display).get_framebuffer_dimensions();

        let mut camera = Camera {
            _focus              : in_focus.to_owned(),
            _theta              : in_theta_degrees*f32::consts::PI/180.0,
            _cos_theta          : Default::default(),
	    _sin_theta          : Default::default(),
	    _phi                : in_phi_degrees*f32::consts::PI/180.0,
	    _cos_phi            : Default::default(),
	    _sin_phi            : Default::default(),
            _psi                : in_psi_degrees*f32::consts::PI/180.0,
	    _cos_psi            : Default::default(),
	    _sin_psi            : Default::default(),
            _r                  : in_r.to_owned(),
            _angular_step       : f32::consts::PI/36.0,
            _r_step             : 0.1,
            _field_of_view      : in_field_of_view_degrees*f32::consts::PI/180.0,
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
    
    fn set_angles(
        &mut self,
        in_theta_degrees : &f32,
        in_phi_degrees   : &f32,
        in_psi_degrees   : &f32,
        in_r             : &f32
    ) {
        self._theta = in_theta_degrees*f32::consts::PI/180.0;
        self._phi = in_phi_degrees*f32::consts::PI/180.0;
	self._psi = in_psi_degrees*f32::consts::PI/180.0;
	self._r = in_r.to_owned();
	self.update();
    }

    fn zoom_in (&mut self) {if self._r > self._r_step {self._r -= self._r_step} self.update();}
    fn zoom_out (&mut self) {self._r += self._r_step; self.update();}
    fn spin_clockwise (&mut self) {
	self._psi += self._angular_step;
	self.update();
    }
    fn spin_anticlockwise (&mut self) {
	self._psi -= self._angular_step;
	self.update();
    }
    fn azimuth_up (&mut self) {
        // let new_sin_theta = self._angular_step.sin()*self._cos_psi*self._cos_theta
        //                   + self._angular_step.cos()*self._sin_theta;
        // let new_cos_theta = (1-new_sin_theta*new_sin_theta).sqrt();
        // let new_sin_psi = self._sin_psi*self._cos_theta/new_cos_theta;

	// implement this using quaternions. Euler angle changes are such a mess.
        self.update();
    }
    fn azimuth_down (&mut self) {
	// implement this using quaternions. Euler angle changes are such a mess.
        self.update();
    }
    fn orbit_right (&mut self) {
	// implement this using quaternions. Euler angle changes are such a mess.
        self.update();
    }
    fn orbit_left (&mut self) {
	// implement this using quaternions. Euler angle changes are such a mess.
        self.update();
    }
    
    fn set_screen_size(&mut self, in_x : &u32, in_y : &u32) {
        self._screen_size = [*in_x, *in_y];
    }
    
    fn update(&mut self) {
        // Update perspective matrix
        let mut w = self._screen_size[0] as f32;
        let mut h = self._screen_size[1] as f32;
        if w > h {
            w = w/h;
            h = 1.0;
        } else {
            h = h/w;
            w = 1.0;
        }
        
        let s = 1.0/(self._field_of_view/2.0).tan();
        let n = self._near_plane.to_owned();
        let f = self._far_plane.to_owned();
        self._perspective_matrix = Matrix::new([
            [s/w, 0.0, 0.0        , 0.0          ],
            [0.0, s/h, 0.0        , 0.0          ],
            [0.0, 0.0, (f+n)/(f-n), 2.0*f*n/(n-f)],
            [0.0, 0.0, 1.0        , 0.0          ]
        ]);
        
        // Update any angles which have gone outside their bounds.
        let pi = f32::consts::PI;
        let hpi = f32::consts::PI/2.0;
        let tpi = 2.0*f32::consts::PI;

	if self._phi > hpi {
	    self._phi = pi-self._phi;
	    self._theta += pi;
	    self._psi += pi;
	}
	if self._phi < -hpi {
	    self._phi = -pi-self._phi;
	    self._theta += pi;
	    self._psi += pi;
	}
	self._theta = (self._theta%tpi+tpi)%tpi;
        self._psi = (self._psi%tpi+tpi)%tpi;

        // Translate so that the focus is centred.
        let focus_translation_matrix = Matrix::new([
            [1.0, 0.0, 0.0, -self._focus[0]],
            [0.0, 1.0, 0.0, -self._focus[1]],
            [0.0, 0.0, 1.0, -self._focus[2]],
            [0.0, 0.0, 0.0,  1.0           ]
        ]);

        // theta is the orbital angle
        self._cos_theta = self._theta.cos();
        self._sin_theta = self._theta.sin();
        let orbital_matrix = Matrix::new([
            [ self._cos_theta, 0.0, self._sin_theta, 0.0],
            [ 0.0            , 1.0, 0.0            , 0.0],
            [-self._sin_theta, 0.0, self._cos_theta, 0.0],
            [ 0.0            , 0.0, 0.0            , 1.0]
        ]);

        // phi is the azimuthal angle
        self._cos_phi = self._phi.cos();
        self._sin_phi = self._phi.sin();
        let azimuthal_matrix = Matrix::new([
            [1.0,  0.0          , 0.0          , 0.0],
            [0.0,  self._cos_phi, self._sin_phi, 0.0],
            [0.0, -self._sin_phi, self._cos_phi, 0.0],
            [0.0,  0.0          , 0.0          , 1.0]
        ]);
        
        // psi is the spin angle
	self._cos_psi = self._psi.cos();
	self._sin_psi = self._psi.sin();
	let spin_matrix = Matrix::new([
	    [ self._cos_psi, self._sin_psi, 0.0, 0.0],
	    [-self._sin_psi, self._cos_psi, 0.0, 0.0],
	    [ 0.0          , 0.0          , 1.0, 0.0],
	    [ 0.0          , 0.0          , 0.0, 1.0]
	]);

	// r is the distance of the camera from the focus
	let zoom_matrix = Matrix::new([
	    [1.0, 0.0, 0.0, 0.0    ],
	    [0.0, 1.0, 0.0, 0.0    ],
	    [0.0, 0.0, 1.0, self._r],
	    [0.0, 0.0, 0.0, 1.0    ]
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
    let display : glium::backend::glutin_backend::GlutinFacade = glium::glutin::WindowBuilder::new()
        .with_title("Furnace: Molecular Visualisation".to_string())
        .build_glium().unwrap();

    // ==============================
    // Make shaders
    // ==============================
    let default_programs = program::DefaultPrograms::new(&display);

    // ==============================
    // Make models
    // ==============================
    let default_models = model::DefaultModels::new(&display, &default_programs);

    // ==============================
    // Make species
    // ==============================
    let default_species = species::DefaultSpecies::new(&default_models);

    // ==============================
    // Make molecule
    // ==============================
    let mut molecule = Molecule::new();
    molecule.add_atom(default_species.sulphur(), &[ 0.0,  0.0, 0.0]);
    molecule.add_atom(default_species.oxygen(), &[ 0.5,  0.5,  0.5]);
    molecule.add_atom(default_species.oxygen(), &[ 0.5, -0.5,  0.5]);
    molecule.add_atom(default_species.oxygen(), &[-0.5,  0.5,  0.5]);
    molecule.add_atom(default_species.nickel(), &[-0.5, -0.5,  0.5]);
    molecule.add_atom(default_species.nickel(), &[ 0.5,  0.5, -0.5]);
    molecule.add_atom(default_species.nickel(), &[ 0.5, -0.5, -0.5]);
    molecule.add_atom(default_species.nickel(), &[-0.5,  0.5, -0.5]);
    molecule.add_atom(default_species.nickel(), &[-0.5, -0.5, -0.5]);
    molecule.add_atom(default_species.carbon(), &[ 0.5,  0.0,  0.0]);
    molecule.add_atom(default_species.carbon(), &[-0.5,  0.0,  0.0]);
    molecule.add_atom(default_species.carbon(), &[ 0.0,  0.5,  0.0]);
    molecule.add_atom(default_species.carbon(), &[ 0.0, -0.5,  0.0]);
    molecule.add_atom(default_species.carbon(), &[ 0.0,  0.0,  0.5]);
    molecule.add_atom(default_species.carbon(), &[ 0.0,  0.0, -0.5]);

    // ==============================
    // Make camera
    // ==============================
    // camera focus (the point the camera is pointing at)
    let camera_focus = [0.0,0.0,0.0];
    // camera position
    let camera_theta_degrees = 0.0;
    let camera_phi_degrees = 0.0;
    let camera_psi_degrees = 0.0;
    let camera_r = 2.0;
    // field of view and clipping planes
    let camera_field_of_view_degrees = 90.0;
    let camera_near_plane = 1.0;
    let camera_far_plane = 10.0;

    let mut camera = Camera::new (
        &display,
        &camera_focus,
	&camera_theta_degrees,
	&camera_phi_degrees,
	&camera_psi_degrees,
	&camera_r,
        &camera_field_of_view_degrees,
        &camera_near_plane,
        &camera_far_plane
    );

    // ==============================
    // Run everything
    // ==============================

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

    let mut fxaa_enabled = true;
    let fxaa = fxaa::FxaaSystem::new(&display);
    loop {
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
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::Resized(x, y) => {
		    camera.set_screen_size(&x, &y);
		},
                
                // ==============================
                // Key is pressed
                // ==============================
                glium::glutin::Event::KeyboardInput (
                    glium::glutin::ElementState::Pressed,
                    _,
                    Some(key)
                ) => match key {
		    glium::glutin::VirtualKeyCode::Escape => return,
		    glium::glutin::VirtualKeyCode::Space => {
                        fxaa_enabled = !fxaa_enabled;
                        println! (
		            "FXAA is now {}",
		            if fxaa_enabled { "on" } else { "off" }
		        );
	            },
		    glium::glutin::VirtualKeyCode::Up => {
		        camera.zoom_in();
			println! ("Zooming in");
		    },
		    glium::glutin::VirtualKeyCode::Down => {
		        camera.zoom_out();
			println!("Zooming out");
		    },
		    glium::glutin::VirtualKeyCode::Right => {
		        camera.spin_clockwise();
			println! ("Spinning clockwise");
		    },
		    glium::glutin::VirtualKeyCode::Left => {
		        camera.spin_anticlockwise();
			println! ("Spinning anticlockwise");
		    },
		    glium::glutin::VirtualKeyCode::K => {
		        camera.azimuth_up();
			println! ("Azimuthing up");
		    },
		    glium::glutin::VirtualKeyCode::J => {
		        camera.azimuth_down();
			println! ("Azimuthing down");
		    },
		    glium::glutin::VirtualKeyCode::H => {
		        camera.orbit_left();
			println! ("Orbiting left");
		    },
		    glium::glutin::VirtualKeyCode::L => {
		        camera.orbit_right();
			println! ("Orbiting right");
		    },
                    glium::glutin::VirtualKeyCode::R => {
                        camera.set_angles (
                            &camera_theta_degrees,
                            &camera_phi_degrees,
                            &camera_psi_degrees,
                            &camera_r
                        );
                        println! ("Resetting camera");
                    },
		    _ => {},
                },

                // ==============================
                // Other
                // ==============================
                _ => ()
            }
        }
    }
}

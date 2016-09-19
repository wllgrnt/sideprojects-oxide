extern crate glium;

use std::f32; // pi

use matrix::Matrix;

// ============================================================
// Camera
// ============================================================
pub struct Camera {
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
    pub fn new (
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
    
    pub fn cos_theta(&self) -> &f32 {&self._cos_theta}
    pub fn sin_theta(&self) -> &f32 {&self._sin_theta}
    pub fn cos_phi(&self) -> &f32 {&self._cos_phi}
    pub fn sin_phi(&self) -> &f32 {&self._sin_phi}
    pub fn cos_psi(&self) -> &f32 {&self._cos_psi}
    pub fn sin_psi(&self) -> &f32 {&self._sin_psi}
    pub fn view_matrix(&self) -> &Matrix {&self._view_matrix}
    pub fn vp_matrix(&self) -> &Matrix {&self._vp_matrix}
    
    pub fn set_angles(
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

    pub fn zoom_in (&mut self) {if self._r > self._r_step {self._r -= self._r_step} self.update();}
    pub fn zoom_out (&mut self) {self._r += self._r_step; self.update();}
    pub fn spin_clockwise (&mut self) {
	self._psi += self._angular_step;
	self.update();
    }
    pub fn spin_anticlockwise (&mut self) {
	self._psi -= self._angular_step;
	self.update();
    }
    pub fn azimuth_up (&mut self) {
        // let new_sin_theta = self._angular_step.sin()*self._cos_psi*self._cos_theta
        //                   + self._angular_step.cos()*self._sin_theta;
        // let new_cos_theta = (1-new_sin_theta*new_sin_theta).sqrt();
        // let new_sin_psi = self._sin_psi*self._cos_theta/new_cos_theta;

	// implement this using quaternions. Euler angle changes are such a mess.
        self.update();
    }
    pub fn azimuth_down (&mut self) {
	// implement this using quaternions. Euler angle changes are such a mess.
        self.update();
    }
    pub fn orbit_right (&mut self) {
	// implement this using quaternions. Euler angle changes are such a mess.
        self.update();
    }
    pub fn orbit_left (&mut self) {
	// implement this using quaternions. Euler angle changes are such a mess.
        self.update();
    }
    
    pub fn set_screen_size(&mut self, in_x : &u32, in_y : &u32) {
        self._screen_size = [*in_x, *in_y];
    }
    
    pub fn update(&mut self) {
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


extern crate glium;

use std::f32; // pi

use matrix::Matrix;
use quaternion::Quaternion;

// ============================================================
// Camera
// ============================================================
pub struct Camera {
    _focus              : [f32;3],
    _r                  : f32,
    _quaternion         : Quaternion,
    _cos_half_step      : f32,
    _sin_half_step      : f32,
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

        let angular_step_radians = f32::consts::PI/36.0;
        let half_step_radians = angular_step_radians/2.0;

        let mut camera = Camera {
            _focus              : in_focus.clone(),
            _r                  : in_r.clone(),
            _quaternion         : Quaternion::new(&1.0,&0.0,&0.0,&0.0),
            _cos_half_step      : half_step_radians.cos(),
            _sin_half_step      : half_step_radians.sin(),
            _r_step             : 0.1,
            _field_of_view      : in_field_of_view_degrees*f32::consts::PI/180.0,
            _near_plane         : in_near_plane.clone(),
            _far_plane          : in_far_plane.clone(),
            _screen_size        : [w, h],
            _view_matrix        : Matrix::new([[0.0;4];4]),   // dummy value
            _perspective_matrix : Matrix::new([[0.0;4];4]),   // dummy value
            _vp_matrix          : Matrix::new([[0.0;4];4]),   // dummy value
        };
        camera.set_angles(in_theta_degrees, in_phi_degrees, in_psi_degrees, in_r);
        camera
    }
    
    pub fn view_matrix(&self) -> &Matrix {&self._view_matrix}
    pub fn perspective_matrix(&self) -> &Matrix {&self._perspective_matrix}
    pub fn vp_matrix(&self) -> &Matrix {&self._vp_matrix}
    pub fn quaternion(&self) -> &Quaternion {&self._quaternion}

    pub fn set_angles(
        &mut self,
        in_theta_degrees : &f32,
        in_phi_degrees   : &f32,
        in_psi_degrees   : &f32,
        in_r             : &f32
    ) {
        let half_theta_radians = in_theta_degrees*f32::consts::PI/360.0;
        let half_phi_radians = in_phi_degrees*f32::consts::PI/360.0;
        let half_psi_radians = in_psi_degrees*f32::consts::PI/360.0;
        self._quaternion = Quaternion::new(
            &half_psi_radians.cos(),
            &0.0,
            &0.0,
            &half_psi_radians.sin(),
        ) * Quaternion::new(
            &half_theta_radians.cos(),
            &half_theta_radians.sin(),
            &0.0,
            &0.0,
        ) * Quaternion::new(
            &half_phi_radians.cos(),
            &0.0,
            &half_phi_radians.sin(),
            &0.0,
        );
        
        self._r = in_r.clone();
        self.update();
    }

    pub fn zoom_in (&mut self) {if self._r > self._r_step {self._r -= self._r_step} self.update();}
    pub fn zoom_out (&mut self) {self._r += self._r_step; self.update();}
    pub fn spin_clockwise (&mut self) {
        self._quaternion.left_multiply(&Quaternion::new(
            &self._cos_half_step,
            &0.0,
            &0.0,
            &-self._sin_half_step
        ));
	    self.update();
    }
    pub fn spin_anticlockwise (&mut self) {
        self._quaternion.left_multiply(&Quaternion::new(
            &self._cos_half_step,
            &0.0,
            &0.0,
            &self._sin_half_step
        ));
	self.update();
    }
    pub fn azimuth_up (&mut self) {
        self._quaternion.left_multiply(&Quaternion::new(
            &self._cos_half_step,
            &self._sin_half_step,
            &0.0,
            &0.0,
        ));
        self.update();
    }
    pub fn azimuth_down (&mut self) {
        self._quaternion.left_multiply(&Quaternion::new(
            &self._cos_half_step,
            &-self._sin_half_step,
            &0.0,
            &0.0,
        ));
        self.update();
    }
    pub fn orbit_right (&mut self) {
        self._quaternion.left_multiply(&Quaternion::new(
            &self._cos_half_step,
            &0.0,
            &-self._sin_half_step,
            &0.0,
        ));
        self.update();
    }
    pub fn orbit_left (&mut self) {
        self._quaternion.left_multiply(&Quaternion::new(
            &self._cos_half_step,
            &0.0,
            &self._sin_half_step,
            &0.0,
        ));
        self.update();
    }
    
    pub fn set_screen_size(&mut self, in_x : &u32, in_y : &u32) {
        self._screen_size = [*in_x, *in_y];
        self.update();
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
        let n = self._near_plane.clone();
        let f = self._far_plane.clone();
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

        self._quaternion.normalise();
        let rotation_matrix = self._quaternion.rotation_matrix();

        // r is the distance of the camera from the focus
        let zoom_matrix = Matrix::new([
            [1.0, 0.0, 0.0, 0.0    ],
            [0.0, 1.0, 0.0, 0.0    ],
            [0.0, 0.0, 1.0, self._r],
            [0.0, 0.0, 0.0, 1.0    ]
        ]);

        self._view_matrix = zoom_matrix
                          * rotation_matrix
                          * focus_translation_matrix;
        self._vp_matrix = self._perspective_matrix*self._view_matrix;
    }
}


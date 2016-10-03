use matrix::Matrix;
use light::Light;
use camera::Camera;

pub struct Lights {
    _lights          : [Light;3],
    _positions       : [[f32;3];3],
    _brightnesses    : [f32;3],
    _rotation_matrix : Matrix,
    _toggle          : bool,
}

impl Lights {
    pub fn new(
        light1 : &Light,
        light2 : &Light,
        light3 : &Light,
    ) -> Lights {
        Lights {
            _lights          : [
                                 light1.clone(),
                                 light2.clone(),
                                 light3.clone(),
                             ],
            _positions       : Default::default(),
            _brightnesses    : [
                                 light1.brightness().clone(),
                                 light2.brightness().clone(),
                                 light3.brightness().clone(),
                             ],
            _rotation_matrix : Matrix::new([[0.0;4];4]),
            _toggle          : true,
        }
    }
    #[allow(dead_code)]
    pub fn lights(&self) -> &[Light;3] {&self._lights}
    #[allow(dead_code)]
    pub fn positions(&self) -> &[[f32;3];3] {&self._positions}
    #[allow(dead_code)]
    pub fn brightnesses(&self) -> &[f32;3] {&self._brightnesses}

    #[allow(dead_code)]
    pub fn set_positions(&mut self, in_camera : &Camera) {
        for i in 0..3 {
            let light_position = if self._toggle {
                *in_camera.view_matrix() * *self._lights[i].position()
            } else {
                self._lights[i].position().clone()
            };
            self._positions[i] = [
                light_position[0],
                light_position[1],
                light_position[2],
            ];
        }
    }
    pub fn toggle(&mut self) {self._toggle = !self._toggle;}
}

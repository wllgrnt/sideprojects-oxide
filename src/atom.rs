use matrix::Matrix;
use species::Species;
use camera::Camera;

// ============================================================
// Atom
// ============================================================
/// The atom, the fundamental unit of a molecular viewer.
pub struct Atom<'a> {
    _species      : &'a Species<'a>,
    _position     : [f32;3],
    _model_matrix : Matrix,
}

impl<'a> Atom<'a> {
    pub fn new (
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

    pub fn species(&self) -> &Species<'a> {&self._species}
    pub fn model_matrix(&self) -> &Matrix {&self._model_matrix}

    pub fn rotate_against_camera(&mut self, in_camera : &Camera) {

        let translation_and_scaling_matrix = Matrix::new ([
            [*self._species.size(), 0.0, 0.0, self._position[0]],
            [0.0, *self._species.size(), 0.0, self._position[1]],
            [0.0, 0.0, *self._species.size(), self._position[2]],
            [0.0, 0.0, 0.0                  , 1.0              ]
        ]);
        
        let mut quaternion = in_camera.quaternion().to_owned();
        quaternion.invert();
        let rotation_matrix = quaternion.rotation_matrix();

        self._model_matrix = translation_and_scaling_matrix * rotation_matrix;
    }
}

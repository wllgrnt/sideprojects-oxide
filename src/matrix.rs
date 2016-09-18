use std::ops::Mul; // multiplication overload

// ============================================================
// Matrix
// ============================================================
// NB: OpenGL (maybe) treats vectors as row vectors, so matrices should be transposed and multiplication reversed?
/// A 4x4 matrix for holding transformations.
#[derive(Copy, Clone)]
pub struct Matrix {
    _contents : [[f32; 4]; 4]
}

impl Matrix {
    pub fn new(in_contents : [[f32; 4]; 4]) -> Matrix {
        Matrix {
            _contents: in_contents
        }
    }

    pub fn contents(&self) -> &[[f32;4];4] {&self._contents}
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


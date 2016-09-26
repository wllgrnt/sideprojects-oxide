pub struct Light {
    _position   : [f32;4],
    _brightness : f32,
}

impl Light {
    pub fn new(in_position : &[f32;3], in_brightness : &f32) -> Light {
        Light{
            _position   : [in_position[0],in_position[1],in_position[2],1.0,],
            _brightness : in_brightness.to_owned(),
        }
    }
    pub fn position(&self) -> &[f32;4] {&self._position}
    pub fn brightness(&self) -> &f32 {&self._brightness}
}

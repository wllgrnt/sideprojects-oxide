use model;
use model::Model;

// ============================================================
// Species
// ============================================================
pub struct Species<'a> {
    _mesh   : &'a Model<'a>,
    _size   : f32,
    _colour : [f32;3],
}

impl<'a> Species<'a> {
    pub fn new (
        in_mesh   : &'a Model,
        in_size   : &f32,
        in_colour : &[f32;3],
    ) -> Species<'a> {
        Species {
            _mesh   : in_mesh,
            _size   : in_size.to_owned(),
            _colour : in_colour.to_owned()
        }
    }

    pub fn mesh(&self) -> &Model {&self._mesh}
    pub fn size(&self) -> &f32  {&self._size}
    pub fn colour(&self) -> &[f32;3] {&self._colour}
}

pub struct DefaultSpecies<'a> {
    _carbon  : Species<'a>,
    _nickel  : Species<'a>,
    _sulphur : Species<'a>,
    _oxygen  : Species<'a>,
}

impl<'a> DefaultSpecies<'a> {
    pub fn new (in_default_models : &'a model::DefaultModels) -> DefaultSpecies<'a> {
        // ==============================
        // Dark2
        // ==============================
        // let turquoise = [ 27.0/255.0,158.0/255.0,119.0/255.0];
        let orange    = [217.0/255.0, 95.0/255.0,  2.0/255.0];
        let blue      = [117.0/255.0,112.0/255.0,179.0/255.0];
        // let pink      = [231.0/255.0, 41.0/255.0,138.0/255.0];
        let green     = [102.0/255.0,166.0/255.0, 30.0/255.0];
        let yellow    = [230.0/255.0,171.0/255.0,  2.0/255.0];
        // let brown     = [166.0/255.0,118.0/255.0, 29.0/255.0];
        // let grey      = [102.0/255.0,102.0/255.0,102.0/255.0];

        DefaultSpecies {
            _carbon  : Species::new(in_default_models.sphere(), &0.1, &blue),
            _nickel  : Species::new(in_default_models.sphere(), &0.2, &orange),
            _sulphur : Species::new(in_default_models.sphere(), &0.4, &yellow),
            _oxygen  : Species::new(in_default_models.sphere(), &0.2, &green),
        }
    }

    pub fn carbon(&self) -> &Species {&self._carbon}
    pub fn nickel(&self) -> &Species {&self._nickel}
    pub fn sulphur(&self) -> &Species {&self._sulphur}
    pub fn oxygen(&self) -> &Species {&self._oxygen}
}

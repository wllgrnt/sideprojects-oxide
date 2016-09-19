use species::Species;
use atom::Atom;
use camera::Camera;

// ============================================================
// Molecule
// ============================================================
// Will likely be the top level struct, unless we need something which has an OpenGL thing + this
/// The molecule. May also be a cluster, crystal motif,...
pub struct Molecule<'a> {
    _atoms : Vec<Atom<'a>>,
}

impl<'a> Molecule<'a> {
    pub fn new() -> Molecule<'a> {Molecule{_atoms : Vec::new()}}

    pub fn add_atom(
        &mut self,
        in_species  : &'a Species,
        in_position : &[f32;3],
    ) {self._atoms.push(Atom::new(in_species, in_position))}

    pub fn atoms(&self) -> &Vec<Atom> {&self._atoms}

    pub fn rotate_atoms_against_camera(&mut self, in_camera : &Camera) {
        for atom in &mut self._atoms {
            atom.rotate_against_camera(in_camera);
        }
    }
}


/// Very basic parser of CASTEP files, returning
/// absolute atomic positions to the main program.
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use molecule::Molecule;
use species::DefaultSpecies;
use model::DefaultModels;

/// Given a valid CASTEP cell file, scrape atomic types, positions and lattice 
/// vectors into memory. Calculate absolute positions and pass them to main.rs
/// to construct the molecule. 
///
/// Example: 
/// cargo run --release test/salt.cell
pub fn read_cell_file<'a>(fname : &String, default_species : &'a DefaultSpecies) -> Molecule<'a> {

    let path = Path::new("test.cell");
    let display = path.display();

    let mut file = File::open(fname).unwrap();
    let mut flines = String::new();

    file.read_to_string(&mut flines);
    println!("{} contains: \n{}", fname, flines);
    let flines : Vec<&str> = flines.split_terminator('\n').collect();

    let mut lattice_cart : Vec<Vec<f32>> = Vec::new();
    let mut positions_frac : Vec<Vec<f32>> = Vec::new();
    let mut species_list : Vec<&str> = Vec::new();

    for (i, line) in flines.iter().enumerate() {
        if line.to_lowercase().trim() == "%block lattice_cart" {
            let mut j = i;
            loop {
                j += 1;
                if flines[j].to_lowercase().trim() == "%endblock lattice_cart" {
                    break;
                } else {
                    let temp : Vec<f32> = flines[j].trim()
                                                   .split_whitespace()
                                                   .map(|s| s.parse::<f32>().unwrap())
                                                   .collect();
                    lattice_cart.push(temp);
                }
            }
        } else if line.to_lowercase() == "%block positions_frac" {
            let mut j = i;
            loop {
                j += 1;
                if flines[j].to_lowercase() == "%endblock positions_frac" {
                    break;
                } else {
                    let temp : Vec<&str> = flines[j].trim().split_whitespace().collect();
                    let mut temp_pos : Vec<f32> = Vec::new();
                    for k in 1..4 {
                        temp_pos.push(temp[k].trim().parse().unwrap());
                    }
                    let atom = temp[0];
                    positions_frac.push(temp_pos);
                    species_list.push(atom);
                }
            }
        }
    }

    println!("Parsed lattice vectors: {:?}", lattice_cart);
    println!("Parsed fractional coordinates: {:?}", positions_frac);
    println!("Parsed atomic species: {:?}", species_list);

    let mut molecule = Molecule::new();

    for (i, atom) in species_list.iter().enumerate() {
        let mut temp_pos : [f32; 3] = [0.0; 3];
        for k in 0..3 {
            for l in 0..3 {
                temp_pos[l] += lattice_cart[k][l] * positions_frac[i][k] - lattice_cart[k][l]/2.0;
            }
        }
        // just stick to oxygen for now
        molecule.add_atom(default_species.oxygen(), &temp_pos);
    }
   return molecule
}

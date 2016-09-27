#[macro_use]
extern crate glium;

mod fxaa;
mod vertex;
mod matrix;
mod quaternion;
mod file_input;
mod model;
mod program;
mod species;
mod atom;
mod molecule;
mod camera;

use glium::{DisplayBuild, Surface};
use molecule::Molecule;
use camera::Camera;
use std::env;

// ============================================================
// Main Program
// ============================================================
/// Furnace - draw a molecule!
fn main() {
    // ==============================
    // Read command-line arguments
    // ==============================
    let args : Vec<String> = env::args().collect();

    // ==============================
    // Make display
    // ==============================
    let display : glium::backend::glutin_backend::GlutinFacade = glium::glutin::WindowBuilder::new()
        .with_title("Oxide: Molecular Visualisation".to_string())
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

    // ==================================
    // Make molecule from file or dummy 
    // ==================================
    let mut molecule = Molecule::new();
    if args.len() > 1 {
        // Load file and, if successful, make models
        println!("Loading {}...", &args[1]);
        let atomic_positions = file_input::read_cell_file(&args[1]);
        for atom in atomic_positions.iter() {
            molecule.add_atom(default_species.oxygen(), &atom);
        }
    } else {
        // Make dummy model if no input 
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
    }
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

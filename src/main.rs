#[macro_use]
extern crate glium;

mod atom;
mod camera;
mod fxaa;
mod light;
mod lights;
mod matrix;
mod model;
mod molecule;
mod program;
mod quaternion;
mod species;
mod vertex;

use glium::{DisplayBuild, Surface};
use molecule::Molecule;
use light::Light;
use lights::Lights;
use camera::Camera;

// ============================================================
// Main Program
// ============================================================
/// Furnace - draw a molecule!
fn main() {
    // ==============================
    // Make display
    // ==============================
    let display = glium::glutin::WindowBuilder::new()
        .with_title("Oxide: Serious Viz-ness".to_string())
        .build_glium().unwrap();

    // ==============================
    // Make programs
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

    // ==============================
    // Make molecule
    // ==============================
    let mut molecule = Molecule::new();
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

    // ==============================
    // Make lights
    // ==============================
    let mut lights = Lights::new(
        &Light::new(&[ 3.0, 0.0, 0.0], &3.0),
        &Light::new(&[-1.0, 1.0, 0.0], &1.0),
        &Light::new(&[ 0.0,-1.0, 1.0], &1.0),
    );

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
    // set up antialiasing
    // ==============================
    let mut fxaa_enabled = true;
    let fxaa = fxaa::FxaaSystem::new(&display);

    // ==============================
    // Run everything
    // ==============================
    loop {

        lights.set_positions(&camera);

        molecule.rotate_atoms_against_camera(&camera);
        
        {
            let mut target : glium::Frame = display.draw();

            let draw_function = |
                mut target : &mut glium::framebuffer::SimpleFrameBuffer
            | {
                target.clear_color_and_depth((0.93, 0.91, 0.835, 1.0), 1.0);
                for atom in molecule.atoms() {
                    atom.draw(&mut target, &lights, &camera);
                }
            };

            fxaa::draw(
                &fxaa,
                &mut target,
                fxaa_enabled,
                draw_function,
            );

            target.finish().unwrap();
        }

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
                        println!("FXAA is now {}", if fxaa_enabled { "on" } else { "off" });
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
                    glium::glutin::VirtualKeyCode::T => {
                        lights.toggle();
                        println!("Toggling lights.");
                    }
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

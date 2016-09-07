#[macro_use]

extern crate glium;

fn main() {

    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    println!("Opening window");
    // Draw a white background screen
    let mut target = display.draw();
    target.clear_color(1.0, 1.0, 1.0, 1.0);
    target.finish().unwrap();
    loop {

        // listing the events produced by the window and waiting to be received
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,   // the window has been closed by the user
                _ => ()
            }
        }
    }

}

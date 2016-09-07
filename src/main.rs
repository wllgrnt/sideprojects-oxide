#[macro_use]
extern crate glium;


/// Furnace - draw a triangle!
fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_title("Furnace: Molecular Visualisation".to_string())
        .build_glium().unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    // The positions of each vertex of the triangle
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    // Create  a vertex buffer for quicker access
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // Dummy index list (since there's only one triangle) 
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
    // Vertex shader in OpenGL v140 (written in GLSL) 
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    // Fragment/Pixel shader in OpenGL v140 (written in GLSL) 
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.847, 0.359375, 0.007812, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.93, 0.91, 0.835, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

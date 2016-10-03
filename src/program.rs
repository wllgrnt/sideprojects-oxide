use atom::Atom;
use camera::Camera;
use lights::Lights;
use model::Model;

use glium;
use glium::Surface;

pub struct Program<'a> {
    _shader_program  : glium::Program,
    _draw_parameters : glium::DrawParameters<'a>,
    _draw_program    : Box<Fn(
                           &mut glium::framebuffer::SimpleFrameBuffer,
                           &Lights,
                           &Camera,
                           &Atom,
                           &Model,
                           &glium::Program,
                           &glium::DrawParameters,
                       )>,
}

impl<'a> Program<'a> {
    pub fn draw(
        &self,
        in_target : &mut glium::framebuffer::SimpleFrameBuffer,
        in_lights : &Lights,
        in_camera : &Camera,
        in_atom   : &Atom,
        in_model  : &Model,
    ) {
        (self._draw_program)(
            in_target,
            in_lights,
            in_camera,
            in_atom,
            in_model,
            &self._shader_program,
            &self._draw_parameters,
        );
    }
}

pub struct DefaultPrograms<'a> {
    _polyhedron : Program<'a>,
    _sphere     : Program<'a>,
}

impl<'a> DefaultPrograms<'a> {
    pub fn new(in_display : &glium::backend::glutin_backend::GlutinFacade) -> DefaultPrograms {
        
        // ====================
        // Polyhedron shaders
        // ====================
        // Vertex shader in OpenGL v140 (written in GLSL)
        let vertex_shader_polyhedron : &'static str = r#"
            #version 140

            uniform mat4 mv_matrix;
            uniform mat4 mvp_matrix;
            uniform vec4 light_position;

            in vec4 _position;
            in vec4 _normal;

            out vec3 fragment_normal;
            out vec3 fragment_light_vector;

            void main() {
                vec4 position = _position*mv_matrix;
                vec4 normal = normalize(_normal*mv_matrix);
                vec4 light_vector = light_position-position;

                fragment_normal = vec3(normal[0],normal[1],normal[2]);
                fragment_light_vector = vec3(light_vector[0],light_vector[1],light_vector[2]);

                gl_Position = _position*mvp_matrix;
            }
        "#;

        // Fragment/Pixel shader in OpenGL v140 (written in GLSL)
        let fragment_shader_polyhedron : &'static str = r#"
            #version 140

            uniform vec3 colour;

            in vec3 fragment_normal;
            in vec3 fragment_light_vector;

            out vec4 color;

            void main() {
                float normal_squared = dot(fragment_normal,fragment_normal);
                float light_distance_squared = dot(fragment_light_vector,fragment_light_vector);
                float cos_light_angle = clamp (
                    dot(fragment_normal,fragment_light_vector)
                        * inversesqrt(light_distance_squared*normal_squared),
                    0,
                    1
                );
                vec3 colour3 = colour*(cos_light_angle/light_distance_squared+0.2);
                color = vec4((colour3), 1.0);
            }
        "#;

        let shader_program_polyhedron = glium::Program::from_source(
            in_display,
            vertex_shader_polyhedron,
            fragment_shader_polyhedron,
            None
        ).unwrap();

        let draw_parameters_polyhedron = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling : glium::BackfaceCullingMode::CullCounterClockwise,
            .. Default::default()
        }; 

        let draw_program_polyhedron = |
            in_target          : &mut glium::framebuffer::SimpleFrameBuffer,
            in_lights          : &Lights,
            in_camera          : &Camera,
            in_atom            : &Atom,
            in_model           : &Model,
            in_shader_program  : &glium::Program,
            in_draw_parameters : &glium::DrawParameters,
        | {
            let mv_matrix = *in_camera.view_matrix() * *in_atom.model_matrix();
            let mvp_matrix = *in_camera.vp_matrix() * *in_atom.model_matrix();
            let perspective_scaling = in_camera.perspective_matrix().contents()[2][3];

            let uniforms = uniform!{
                mv_matrix : mv_matrix.contents().clone(),
                mvp_matrix : mvp_matrix.contents().clone(),
                base_colour : in_atom.species().colour().clone(),
                light_positions : in_lights.positions().clone(),
                light_brightnesses : in_lights.brightnesses().clone(),
                size : in_atom.species().size().clone(),
                eye_space_depth : mv_matrix.contents()[2][3],
                perspective_scaling : perspective_scaling,
            };

            in_target.draw(
                in_model.vertex_buffer(),
                in_model.index_buffer(),
                in_shader_program,
                &uniforms,
                in_draw_parameters,
            ).unwrap();
        };

        // ====================
        // Sphere shaders
        // ====================
        // Vertex shader in OpenGL v140 (written in GLSL)
        let vertex_shader_sphere : &'static str = r#"
            #version 140

            uniform mat4 mv_matrix;
            uniform mat4 mvp_matrix;

            in vec4 _position;
            in vec4 _normal;
            
            out vec2 fragment_xy;
            out vec3 fragment_position;

            void main() {
                fragment_xy = _normal.xy;
                fragment_position = (_position*mv_matrix).xyz;

                gl_Position = _position*mvp_matrix;
            }
        "#;

        // Fragment/Pixel shader in OpenGL v140 (written in GLSL)
        let fragment_shader_sphere : &'static str = r#"
            #version 140

            uniform vec3 base_colour;
            uniform float size;                // the radius of the sphere
            uniform float eye_space_depth;     // the z coordinate of the centre of the sphere in eye space
            uniform float perspective_scaling; // the [2][3] element of the perspective matrix
            uniform mat3 light_positions;
            uniform vec3 light_brightnesses;

            in vec2 fragment_xy;
            in vec3 fragment_position;

            out vec4 color;

            void main() {
                float xy_squared = dot(fragment_xy,fragment_xy);
                if (xy_squared > 1)
                    discard;
                vec3 normal = vec3(fragment_xy[0],fragment_xy[1],-sqrt(1-xy_squared));
                float depth_change = -size*normal[2]; // positive, because normal[2] is negative

                float brightness = 0.05; // ambient lighting
                
                // diffuse lighting
                for (int i=0; i<3; ++i) {
                    vec3 light_vector = light_positions[i]-fragment_position;
                    light_vector[2] += depth_change;
                    float light_distance_squared = dot(light_vector, light_vector);
                    float cos_light_angle = clamp (
                        dot(normal,light_vector) * inversesqrt(light_distance_squared),
                        0,
                        1
                    );
                    brightness += light_brightnesses[i]*cos_light_angle/light_distance_squared;
                }
                
                // set the colour of the fragment
                color = vec4(base_colour*brightness, 1.0);

                // correct the z-position of the fragment
                // involves messy transformation from eye space to clip space to screen space.
                gl_FragDepth = gl_FragCoord[2]
                             + depth_change*perspective_scaling*(gl_DepthRange.far-gl_DepthRange.near)
                             / (2.0*eye_space_depth*(eye_space_depth-depth_change));
            }
        "#;

        let shader_program_sphere = glium::Program::from_source(
            in_display,
            vertex_shader_sphere,
            fragment_shader_sphere,
            None
        ).unwrap();
        
        let draw_parameters_sphere = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling : glium::BackfaceCullingMode::CullCounterClockwise,
            .. Default::default()
        };

        let draw_program_sphere = |
            in_target          : &mut glium::framebuffer::SimpleFrameBuffer,
            in_lights          : &Lights,
            in_camera          : &Camera,
            in_atom            : &Atom,
            in_model           : &Model,
            in_shader_program  : &glium::Program,
            in_draw_parameters : &glium::DrawParameters,
        | {
            let mv_matrix = *in_camera.view_matrix() * *in_atom.model_matrix();
            let mvp_matrix = *in_camera.vp_matrix() * *in_atom.model_matrix();
            let perspective_scaling = in_camera.perspective_matrix().contents()[2][3];

            let uniforms = uniform!{
                mv_matrix : mv_matrix.contents().clone(),
                mvp_matrix : mvp_matrix.contents().clone(),
                base_colour : in_atom.species().colour().clone(),
                light_positions : in_lights.positions().clone(),
                light_brightnesses : in_lights.brightnesses().clone(),
                size : in_atom.species().size().clone(),
                eye_space_depth : mv_matrix.contents()[2][3],
                perspective_scaling : perspective_scaling,
            };

            in_target.draw(
                in_model.vertex_buffer(),
                in_model.index_buffer(),
                in_shader_program,
                &uniforms,
                in_draw_parameters,
            ).unwrap();
        };

        DefaultPrograms {
            _polyhedron : Program{
                _shader_program  : shader_program_polyhedron,
                _draw_parameters : draw_parameters_polyhedron,
                _draw_program    : Box::new(draw_program_polyhedron),
            },
            _sphere : Program{
                _shader_program  : shader_program_sphere,
                _draw_parameters : draw_parameters_sphere,
                _draw_program    : Box::new(draw_program_sphere),
            },
        }
    }

    pub fn polyhedron(&self) -> &Program {&self._polyhedron}
    pub fn sphere(&self) -> &Program {&self._sphere}
}

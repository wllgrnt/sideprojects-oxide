extern crate glium;

pub struct DefaultPrograms {
    _polyhedron : glium::Program,
    _sphere     : glium::Program,
}

impl DefaultPrograms {
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
        
        DefaultPrograms {
            _polyhedron : glium::Program::from_source(
                in_display,
                vertex_shader_polyhedron,
                fragment_shader_polyhedron,
                None
            ).unwrap(),
            _sphere : glium::Program::from_source(
                in_display,
                vertex_shader_sphere,
                fragment_shader_sphere,
                None
            ).unwrap(),
        }
    }

    pub fn polyhedron(&self) -> &glium::Program {&self._polyhedron}
    pub fn sphere(&self) -> &glium::Program {&self._sphere}
}

use gl;
use std::mem;
use std::os::raw;
use window;
use input;
use window_gl;
use render::buffer;
use render_gl::{vao, shader, texture};
use resources::Resources;
use cgmath as m;
use cgmath::prelude::*;

/// http://learnopengl.com/#!Getting-started/Transformations
pub struct Cube {
    shader: shader::Program,
    texture1: texture::Texture,
    texture2: texture::Texture,
    data: Vec<gl::types::GLfloat>,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
    time_value: f32,
    positions: Vec<m::Vector3<f32>>,
}

impl Cube {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new() -> Cube {

        Cube {
            shader: shader::Program::new("cube"),
            texture1: texture::Texture::new("stone.jpg"),
            texture2: texture::Texture::new("rust.png"),
            data: vec![
                -0.5, -0.5, -0.5,  0.0, 0.0,
                0.5, -0.5, -0.5,  1.0, 0.0,
                0.5,  0.5, -0.5,  1.0, 1.0,
                0.5,  0.5, -0.5,  1.0, 1.0,
                -0.5,  0.5, -0.5,  0.0, 1.0,
                -0.5, -0.5, -0.5,  0.0, 0.0,

                -0.5, -0.5,  0.5,  0.0, 0.0,
                0.5, -0.5,  0.5,  1.0, 0.0,
                0.5,  0.5,  0.5,  1.0, 1.0,
                0.5,  0.5,  0.5,  1.0, 1.0,
                -0.5,  0.5,  0.5,  0.0, 1.0,
                -0.5, -0.5,  0.5,  0.0, 0.0,

                -0.5,  0.5,  0.5,  1.0, 0.0,
                -0.5,  0.5, -0.5,  1.0, 1.0,
                -0.5, -0.5, -0.5,  0.0, 1.0,
                -0.5, -0.5, -0.5,  0.0, 1.0,
                -0.5, -0.5,  0.5,  0.0, 0.0,
                -0.5,  0.5,  0.5,  1.0, 0.0,

                0.5,  0.5,  0.5,  1.0, 0.0,
                0.5,  0.5, -0.5,  1.0, 1.0,
                0.5, -0.5, -0.5,  0.0, 1.0,
                0.5, -0.5, -0.5,  0.0, 1.0,
                0.5, -0.5,  0.5,  0.0, 0.0,
                0.5,  0.5,  0.5,  1.0, 0.0,

                -0.5, -0.5, -0.5,  0.0, 1.0,
                0.5, -0.5, -0.5,  1.0, 1.0,
                0.5, -0.5,  0.5,  1.0, 0.0,
                0.5, -0.5,  0.5,  1.0, 0.0,
                -0.5, -0.5,  0.5,  0.0, 0.0,
                -0.5, -0.5, -0.5,  0.0, 1.0,

                -0.5,  0.5, -0.5,  0.0, 1.0,
                0.5,  0.5, -0.5,  1.0, 1.0,
                0.5,  0.5,  0.5,  1.0, 0.0,
                0.5,  0.5,  0.5,  1.0, 0.0,
                -0.5,  0.5,  0.5,  0.0, 0.0,
                -0.5,  0.5, -0.5,  0.0, 1.0
            ],
            vbo: 0,
            vao: 0,
            time_value: 0.0,
            positions: vec![
                m::vec3( 0.0,  0.0,  0.0),
                m::vec3( 2.0,  5.0, -15.0),
                m::vec3(-1.5, -2.2, -2.5),
                m::vec3(-3.8, -2.0, -12.3),
                m::vec3( 2.4, -0.4, -3.5),
                m::vec3(-1.7,  3.0, -7.5),
                m::vec3( 1.3, -2.0, -2.5),
                m::vec3( 1.5,  2.0, -2.5),
                m::vec3( 1.5,  0.2, -1.5),
                m::vec3(-1.3,  1.0, -1.5)
            ],
        }
    }
}

impl input::Handle<window_gl::Info> for Cube {}

impl window::Render<window_gl::Info> for Cube {
    fn init(&mut self, window: &window_gl::Info, resources: &Resources) {
        let gl = &window.gl;

        unsafe {
            gl.Enable(gl::DEPTH_TEST);
        }

        self.shader.init(gl, resources);
        self.texture1.init(gl, resources);
        self.texture2.init(gl, resources);
        unsafe {
            gl.GenVertexArrays(1, &mut self.vao);
            gl.GenBuffers(1, &mut self.vbo);

            // 1. Bind Vertex Array Object
            gl.BindVertexArray(self.vao);

            // 2. Copy our vertices array in a buffer for OpenGL to use
            gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl.BufferData(gl::ARRAY_BUFFER,
                          (mem::size_of::<gl::types::GLfloat>() *
                              self.data.len()) as gl::types::GLsizeiptr,
                          self.data.as_mut_ptr() as *const raw::c_void,
                          gl::STATIC_DRAW);

            // 3. Then set our vertex attributes pointers
            let layout = buffer::Layout::new()
                // Position attribute
                .with(0, buffer::Format::f32_f32_f32, buffer::Padding::p0)
                // Texture coords
                .with(1, buffer::Format::f32_f32, buffer::Padding::p0);
            vao::attrib_pointers(gl, &layout);

            // 4. Unbind the VAO
            gl.BindVertexArray(0);
        }
    }

    fn update(&mut self, delta: f32) {
        self.time_value += delta;
    }

    fn render(&self, window: &window_gl::Info) {
        let gl = &window.gl;

        let angle2 = 20.0 as f32 + self.time_value * 50.0;

        let view = m::Matrix4::look_at(m::Point3::new(0.0, 0.0, 3.0),
                                       m::Point3::new(0.0, 0.0, 0.0),
                                       m::vec3(0.0, 1.0, 0.0)) *
            m::Matrix4::from_axis_angle(m::vec3(1.0, 0.3, 0.5).normalize(),
                                        m::Deg(angle2));

        let projection: m::Matrix4<f32> = m::perspective::<f32, m::Deg<f32>>(m::Deg(45.0).into(),
                                                                             window.window.width as f32 /
                                                                                 window.window.height as f32,
                                                                             0.1,
                                                                             100.0);

        self.shader.use_program(gl);

        self.texture1.bind_at(gl, 0);
        self.shader.set_named_uniform1i(gl, "ourTexture1", 0);
        self.texture2.bind_at(gl, 1);
        self.shader.set_named_uniform1i(gl, "ourTexture2", 1);


        self.shader.set_named_uniform_matrix4fv(gl, "view", &view);
        self.shader.set_named_uniform_matrix4fv(gl, "projection", &projection);

        unsafe {
            gl.BindVertexArray(self.vao);
        }

        for (i, &position) in self.positions.iter().enumerate() {
            let angle1 = -20.0 * i as f32 -
                self.time_value * 30.0;
            let angle2 = 20.0 * i as f32 +
                self.time_value * 50.0;

            let model = m::Matrix4::from_axis_angle(m::vec3(0.1, 0.3, 1.0).normalize(),
                                                    m::Deg(angle1)) *
                m::Matrix4::from_translation(position) *
                m::Matrix4::from_axis_angle(m::vec3(1.0, 0.3, 0.5).normalize(),
                                            m::Deg(angle2));

            self.shader.set_named_uniform_matrix4fv(gl, "model", &model);

            unsafe {
                gl.DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }


        unsafe {
            gl.BindVertexArray(0);
        }
    }
}
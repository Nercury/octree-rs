use cgmath::*;
use std::cell::RefCell;
use gl::{self, Glw};
use resources::Resources;
use shader::Program;
use render::{buffer};
use vec_2_10_10_10;

struct RenderState {
    shader: Option<Program>,
    last_len: usize,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
    lines: Vec<LineItem>,
}

impl RenderState {
    pub fn new() -> RenderState {
        RenderState { shader: None, last_len: 0, vbo: 0, vao: 0, lines: Vec::new() }
    }

    pub fn maybe_init(&mut self, gl: &Glw) {
        if self.last_len != self.lines.len() {
            println!("lines {:?}", &self.lines);

            unsafe {
                if self.vao != 0 {
                    gl.DeleteVertexArrays(1, &mut self.vao);
                }

                if self.vbo != 0 {
                    gl.DeleteBuffers(1, &mut self.vbo);
                }

                gl.GenVertexArrays(1, &mut self.vao);
                gl.GenBuffers(1, &mut self.vbo);

                // 1. Bind Vertex Array Object
                gl.BindVertexArray(self.vao);

                let mut len = self.lines.len();
                if len % 2 != 0 {
                    len -= 1;
                }

                // 2. Copy our vertices array in a buffer for OpenGL to use
                gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                gl.BufferData(gl::ARRAY_BUFFER,
                              (::std::mem::size_of::<f32>() *
                                  len * 8) as gl::types::GLsizeiptr,
                              self.lines.as_mut_ptr() as *const ::std::os::raw::c_void,
                              gl::STATIC_DRAW);

                let layout = buffer::Layout::new()
                    .with(0, buffer::Format::f32_f32_f32, buffer::Padding::p0)
                    .with(1, buffer::Format::u2_u10_u10_u10_rev_float, buffer::Padding::p0);

                ::vao::attrib_pointers(gl, &layout);

                gl.BindVertexArray(0);
            }

            self.last_len = self.lines.len();
        }
    }
}

pub struct DebugLines {
    state: RefCell<RenderState>,
}

impl DebugLines {
    pub fn init(&mut self, gl: &Glw, resources: &Resources) {
        self.state.borrow_mut().shader = Some({
            let mut p = Program::new("util_debug_lines");
            p.init(gl, resources);
            p
        });
    }

    pub fn render(&self, gl: &Glw) {
        let mut state = self.state.borrow_mut();
        state.maybe_init(gl);

        if state.lines.len() > 0 {
            state.shader.as_ref().unwrap().use_program(gl);
            unsafe {
                gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                gl.Enable(gl::BLEND);
                gl.BindVertexArray(state.vao);
                gl.DrawArrays(gl::LINES, 0, state.lines.len() as i32);
                gl.BindVertexArray(0);
                gl.Disable(gl::BLEND);
            }
        }
    }

    pub fn new() -> DebugLines {
        DebugLines {
            state: RefCell::new(RenderState::new()),
        }
    }

    pub fn clear(&mut self) {
        self.state.borrow_mut().lines.truncate(0);
    }

    pub fn from_vec3(&mut self, from: Vector3<f32>) -> LineBuilder {
        {
            let ref mut lines = self.state.borrow_mut().lines;
            lines.push(LineItem {
                x: from.x,
                y: from.y,
                z: from.z,
                c: vec_2_10_10_10::Vector::new(1.0, 1.0, 1.0, 1.0),
            });
        }
        LineBuilder {
            first_line: true,
            debug_lines: self,
        }
    }

    pub fn from(&mut self, x: f32, y: f32, z: f32) -> LineBuilder {
        self.from_vec3(vec3(x, y, z))
    }
}

pub struct LineBuilder<'a> {
    first_line: bool,
    debug_lines: &'a mut DebugLines,
}

impl<'a> LineBuilder<'a> {
    pub fn color_vec3(self, color: Vector3<f32>) -> Self {
        self.color3(color.x, color.y, color.z)
    }

    pub fn color_vec4(self, color: Vector4<f32>) -> Self {
        self.color4(color.x, color.y, color.z, color.w)
    }

    pub fn alpha(self, a: f32) -> Self {
        self.debug_lines.state.borrow_mut().lines
            .last_mut()
            .unwrap().c.set_w(a);
        self
    }

    pub fn color3(self, r: f32, g: f32, b: f32) -> Self {
            self.debug_lines.state.borrow_mut().lines
                .last_mut()
                .unwrap().c.set_xyz(r, g, b);
        self
    }

    pub fn color4(self, r: f32, g: f32, b: f32, a: f32) -> Self {
            self.debug_lines.state.borrow_mut().lines
                .last_mut()
                .unwrap().c = vec_2_10_10_10::Vector::new(r, g, b, a);
        self
    }

    pub fn to_vec3(mut self, b: Vector3<f32>) -> Self {
        {
            let lines = &mut self.debug_lines.state.borrow_mut().lines;

            let mut item = *lines
                .last()
                .unwrap();

            if !self.first_line {
                lines.push(item);
            }

            item.x = b.x;
            item.y = b.y;
            item.z = b.z;
            lines.push(item);
            self.first_line = false;
        }

        self
    }

    pub fn to(self, x: f32, y: f32, z: f32) -> Self {
        self.to_vec3(vec3(x, y, z))
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct LineItem {
    x: f32,
    y: f32,
    z: f32,
    c: vec_2_10_10_10::Vector,
}
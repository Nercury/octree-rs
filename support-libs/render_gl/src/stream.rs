use gl::types::*;
use gl::{self, Glw};
use vertex_source::{VertexSource, NextFrame};
use intptr_size_of;
use check_err;
use std::ptr;
use std::os::raw;
use std::slice;

pub struct Stream<T> {
    vertex_source: T,
    vbo_size: usize,
    ebo_size: usize,
    vbo_start_byte: usize,
    ebo_start_byte: usize,
    vbo_end_byte: usize,
    ebo_end_byte: usize,
    ebo_element_len: usize,
    vbo: GLuint,
    vao: GLuint,
    ebo: GLuint,
}

impl<T: VertexSource> Stream<T> {
    pub fn new(vertex_source: T) -> Stream<T> {
        Stream {
            vertex_source: vertex_source,
            vbo_size: 0,
            ebo_size: 0,
            vbo_start_byte: 0,
            ebo_start_byte: 0,
            vbo_end_byte: 0,
            ebo_end_byte: 0,
            ebo_element_len: 0,
            vbo: 0,
            vao: 0,
            ebo: 0,
        }
    }

    pub fn init(&mut self, gl: &Glw) {
        self.deinit(gl);
        unsafe {
            gl.GenVertexArrays(1, &mut self.vao);
            gl.GenBuffers(1, &mut self.vbo);
            gl.GenBuffers(1, &mut self.ebo);

            self.bind_vao(gl);
        }
    }

    pub fn deinit(&mut self, gl: &Glw) {
        if unsafe { gl.IsVertexArray(self.vao) } == gl::TRUE {
            unsafe { gl.DeleteVertexArrays(1, &mut self.vao) };
        }
        if unsafe { gl.IsBuffer(self.vbo) } == gl::TRUE {
            unsafe { gl.DeleteBuffers(1, &mut self.vbo) };
        }
        if unsafe { gl.IsBuffer(self.ebo) } == gl::TRUE {
            unsafe { gl.DeleteBuffers(1, &mut self.ebo) };
        }

        self.vbo = 0;
        self.vao = 0;
        self.ebo = 0;
        self.vbo_start_byte = 0;
        self.ebo_start_byte = 0;
        self.vbo_end_byte = 0;
        self.ebo_end_byte = 0;
        self.ebo_element_len = 0;
    }

    pub fn update(&mut self, gl: &Glw, delta: f32) {
        if let Some(next_frame) = self.vertex_source.update(delta) {
            self.validate_or_adjust(gl, &next_frame);

            if next_frame.vbo_size > 0 && next_frame.ebo_size > 0 {
                unsafe {
                    let vbo_start = self.vbo_end_byte;

                    gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                    let vbo_ptr = gl.MapBufferRange(gl::ARRAY_BUFFER,
                                                    self.vbo_end_byte as isize,
                                                    next_frame.vbo_size as isize,
                                                    gl::MAP_WRITE_BIT |
                                                        gl::MAP_INVALIDATE_RANGE_BIT |
                                                        gl::MAP_UNSYNCHRONIZED_BIT);
                    if vbo_ptr.is_null() {
                        check_err(gl);
                        panic!("failed to map vbo");
                    }
                    let vbo_slice = slice::from_raw_parts_mut(vbo_ptr as *mut u8,
                                                              next_frame.vbo_size);

                    self.vbo_start_byte = self.vbo_end_byte;
                    self.vbo_end_byte += next_frame.vbo_size;

                    gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
                    let ebo_ptr = gl.MapBufferRange(gl::ELEMENT_ARRAY_BUFFER,
                                                    self.ebo_end_byte as isize,
                                                    next_frame.ebo_size as isize,
                                                    gl::MAP_WRITE_BIT |
                                                        gl::MAP_INVALIDATE_RANGE_BIT |
                                                        gl::MAP_UNSYNCHRONIZED_BIT);
                    if ebo_ptr.is_null() {
                        check_err(gl);
                        panic!("failed to map ebo");
                    }
                    let ebo_slice = slice::from_raw_parts_mut(ebo_ptr as *mut u8,
                                                              next_frame.ebo_size);

                    self.ebo_start_byte = self.ebo_end_byte;
                    self.ebo_end_byte += next_frame.ebo_size;

                    self.ebo_element_len = self.vertex_source
                        .fill(vbo_start, vbo_slice, ebo_slice);

                    gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                    gl.UnmapBuffer(gl::ARRAY_BUFFER);
                    gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
                    gl.UnmapBuffer(gl::ELEMENT_ARRAY_BUFFER);
                }
            }
        }
    }

    fn bind_vao(&mut self, gl: &Glw) {
        unsafe {
            gl.BindVertexArray(self.vao);

            gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl.BufferData(gl::ARRAY_BUFFER,
                          intptr_size_of::<GLfloat>(self.vbo_size),
                          ptr::null_mut(),
                          gl::STREAM_DRAW);

            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl.BufferData(gl::ELEMENT_ARRAY_BUFFER,
                          intptr_size_of::<GLfloat>(self.ebo_size),
                          ptr::null_mut(),
                          gl::STREAM_DRAW);

            self.vertex_source.vao_init(gl);

            gl.BindVertexArray(0);
        }

        self.vbo_start_byte = 0;
        self.ebo_start_byte = 0;
        self.vbo_end_byte = 0;
        self.ebo_end_byte = 0;
    }

    fn validate_or_adjust(&mut self, gl: &Glw, next_frame: &NextFrame) {

        if next_frame.vbo_size == 0 || next_frame.ebo_size == 0 {
            self.vbo_size = 0;
            self.ebo_size = 0;
            self.vbo_end_byte = 0;
            self.vbo_start_byte = 0;
            self.ebo_end_byte = 0;
            self.ebo_start_byte = 0;
            self.bind_vao(gl);
            return;
        }

        if self.vbo_size < next_frame.vbo_size * 4 || self.ebo_size < next_frame.ebo_size * 4 {
            if self.vbo_size < next_frame.vbo_size {
                let preferred_size = if self.vbo_size == 0 {
                    next_frame.vbo_size * 4
                } else {
                    self.vbo_size * 2
                };

                self.vbo_size = preferred_size;
                println!("vbo size adjusted to {:?}", self.vbo_size);
            }
            if self.ebo_size < next_frame.ebo_size * 4 {
                let preferred_size = if self.ebo_size == 0 {
                    next_frame.ebo_size * 4
                } else {
                    self.ebo_size * 2
                };

                self.ebo_size = preferred_size;
                println!("ebo size adjusted to {:?}", self.ebo_size);
            }
            self.bind_vao(gl);
        }

        let new_range_end = self.vbo_end_byte + next_frame.vbo_size;
        if new_range_end > self.vbo_size {
            if self.vbo_end_byte > 0 && next_frame.vbo_size > self.vbo_start_byte {
                panic!("reusing same vbo buffer would require {} bytes which would overwrite a \
                        previous range that starts at {} and uses {} bytes",
                       next_frame.vbo_size,
                       self.vbo_start_byte,
                       self.vbo_end_byte - self.vbo_start_byte);
            }

            self.vbo_end_byte = 0;
            self.vbo_start_byte = 0;
        }

        let new_range_end = self.ebo_end_byte + next_frame.ebo_size;
        if new_range_end > self.ebo_size {
            if self.ebo_end_byte > 0 && next_frame.ebo_size > self.ebo_start_byte {
                panic!("reusing same ebo buffer would require {} bytes which would overwrite a \
                        previous range that starts at {} and uses {} bytes",
                       next_frame.ebo_size,
                       self.ebo_start_byte,
                       self.ebo_end_byte - self.ebo_start_byte);
            }

            self.ebo_end_byte = 0;
            self.ebo_start_byte = 0;
        }
    }

    pub fn render(&self, gl: &Glw) {
        unsafe {
            gl.BindVertexArray(self.vao);
            gl.DrawElements(gl::TRIANGLES,
                            self.ebo_element_len as GLint,
                            gl::UNSIGNED_INT,
                            self.ebo_start_byte as *const raw::c_void);
            gl.BindVertexArray(0);
        }
    }
}

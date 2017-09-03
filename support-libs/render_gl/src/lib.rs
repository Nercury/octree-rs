extern crate sdl2;
extern crate gl_core_struct as gl;
extern crate ffh;
extern crate cgmath;
extern crate resources;
extern crate image;
extern crate render;
extern crate vec_2_10_10_10;

pub mod shader;
pub mod texture;
pub mod stream;
pub mod vertex_source;
pub mod gl_writer;
pub mod vao;
pub mod util;

use std::mem;
use gl::Gl;
use gl::types::*;

pub fn intptr_size_of<T>(mul: usize) -> GLsizeiptr {
    (mem::size_of::<T>() * mul) as GLsizeiptr
}

pub fn int_size_of<T>(mul: usize) -> GLint {
    (mem::size_of::<T>() * mul) as GLint
}

pub fn void_size_of<T>(mul: usize) -> *const GLvoid {
    (mem::size_of::<T>() * mul) as *const GLvoid
}

pub fn check_err(gl: &Gl) -> bool {
    let res = unsafe { gl.GetError() };
    if res == gl::NO_ERROR {
        return false;
    }

    println!("GL error {}: {}",
             res,
             match res {
                 gl::NO_ERROR => {
                     "NO_ERROR = No error has been recorded.
                        The value of this \
                      symbolic constant is guaranteed to be 0."
                 }
                 gl::INVALID_ENUM => {
                     "INVALID_ENUM = An unacceptable value is specified for an enumerated argument.
                        \
                      The offending command is ignored
                        and has no other \
                      side effect than to set the error flag."
                 }
                 gl::INVALID_VALUE => {
                     "INVALID_VALUE = A numeric argument is out of range.
                        The offending command is ignored
                        and has no other side effect than to set the error flag."
                 }
                 gl::INVALID_OPERATION => {
                     "INVALID_OPERATION = The specified operation is not allowed in the current \
                      state.
                        The offending command is ignored
                        \
                      and has no other side effect than to set the error flag."
                 }
                 gl::INVALID_FRAMEBUFFER_OPERATION => {
                     "INVALID_FRAMEBUFFER_OPERATION = The command is trying to render to or read \
                      from the framebuffer
                        while the currently bound \
                      framebuffer is not framebuffer
                        complete (i.e. the \
                      return value from
                        glCheckFramebufferStatus
                        \
                      is not GL_FRAMEBUFFER_COMPLETE).
                        The offending \
                      command is ignored
                        and has no other side effect than \
                      to set the error flag."
                 }
                 gl::OUT_OF_MEMORY => {
                     "OUT_OF_MEMORY = There is not enough memory left to execute the command.
                        The state of the GL is undefined,
                        except for the state of the error flags,
                        after this error is recorded."
                 }
                 _ => "Unknown error",
             });
    true
}

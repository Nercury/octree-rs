use gl::{self, Gl};
use std::ptr;
use std::ffi::CString;
use std::os::raw;
use ffh;
use cgmath as m;
use resources;

pub enum ProgramKind {
    VertexAndFragment,
}

pub struct Program {
    name: String,
    program: Option<gl::types::GLuint>,
    kind: ProgramKind,
}

impl Program {
    pub fn new(name: &str) -> Program {
        Program {
            name: name.to_string(),
            kind: ProgramKind::VertexAndFragment,
            program: None,
        }
    }

    pub fn set_uniform_4f(&self, gl: &Gl, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        if let Some(program) = self.program {
            let mut bytes = [b' ' as u8; 2048];
            let cstr = ffh::str_to_cstr(&mut bytes, name);

            unsafe {
                let uniform_loc = gl.GetUniformLocation(program, cstr.as_ptr());
                gl.Uniform4f(uniform_loc, v0, v1, v2, v3);
            }
        }
    }

    pub fn set_named_uniform1i(&self, gl: &Gl, name: &str, index: gl::types::GLint) {
        if let Some(id) = self.program {
            let mut bytes = [b' ' as u8; 2048];
            let cstr = ffh::str_to_cstr(&mut bytes, name);
            unsafe {
                gl.Uniform1i(gl.GetUniformLocation(id, cstr.as_ptr()), index);
            }
        }
    }

    pub fn set_named_uniform3ui(&self, gl: &Gl, name: &str, v0: u32, v1: u32, v2: u32) {
        if let Some(id) = self.program {
            let mut bytes = [b' ' as u8; 2048];
            let cstr = ffh::str_to_cstr(&mut bytes, name);
            unsafe {
                gl.Uniform3ui(gl.GetUniformLocation(id, cstr.as_ptr()), v0, v1, v2);
            }
        }
    }

    pub fn set_named_uniform3f(&self, gl: &Gl, name: &str, v0: f32, v1: f32, v2: f32) {
        if let Some(id) = self.program {
            let mut bytes = [b' ' as u8; 2048];
            let cstr = ffh::str_to_cstr(&mut bytes, name);
            unsafe {
                gl.Uniform3f(gl.GetUniformLocation(id, cstr.as_ptr()), v0, v1, v2);
            }
        }
    }

    pub fn set_named_uniform2f(&self, gl: &Gl, name: &str, v0: f32, v1: f32) {
        if let Some(id) = self.program {
            let mut bytes = [b' ' as u8; 2048];
            let cstr = ffh::str_to_cstr(&mut bytes, name);
            unsafe {
                gl.Uniform2f(gl.GetUniformLocation(id, cstr.as_ptr()), v0, v1);
            }
        }
    }

    pub fn set_named_uniform_matrix4fv(&self, gl: &Gl, name: &str, matrix: &m::Matrix4<f32>) {
        if let Some(id) = self.program {
            let mut bytes = [b' ' as u8; 2048];
            let cstr = ffh::str_to_cstr(&mut bytes, name);
            unsafe {
                let transform_loc = gl.GetUniformLocation(id, cstr.as_ptr());
                gl.UniformMatrix4fv(transform_loc,
                                    1,
                                    gl::FALSE,
                                    m::conv::array4x4(*matrix).as_ptr() as *const f32);
            }
        }
    }

    fn create_shader(&self,
                     gl: &Gl,
                     source: &CString,
                     kind: gl::types::GLuint)
                     -> Result<gl::types::GLuint, String> {
        let shader = unsafe { gl.CreateShader(kind) };
        unsafe {
            gl.ShaderSource(shader, 1, &source.as_ptr(), ptr::null());
            gl.CompileShader(shader);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        }


        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error: CString =
                unsafe { CString::from_vec_unchecked(vec![b' ' as u8; len as usize]) };

            unsafe {
                gl.GetShaderInfoLog(shader,
                                    len,
                                    ptr::null_mut(),
                                    error.as_ptr() as *mut raw::c_char);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(shader)
    }

    pub fn deinit(&mut self, gl: &Gl) {
        if let Some(id) = self.program {
            if unsafe { gl.IsProgram(id) } == gl::TRUE {
                unsafe { gl.DeleteProgram(id) };
            }
        }
        self.program = None;
    }

    pub fn use_program(&self, gl: &Gl) {
        if let Some(program) = self.program {
            unsafe {
                gl.UseProgram(program);
            }
        }
    }

    pub fn init(&mut self, gl: &Gl, resources: &resources::Resources) {
        self.deinit(gl);

        match self.kind {
            ProgramKind::VertexAndFragment => {
                let vname = ["shader/", &*self.name, ".vert"].concat();
                let fname = ["shader/", &*self.name, ".frag"].concat();

                let vsource = resources.get_cstring(&vname);
                let fsource = resources.get_cstring(&fname);

                let vshader = match self.create_shader(gl, &vsource, gl::VERTEX_SHADER) {
                    Ok(sh) => Some(sh),
                    Err(stuff) => {
                        println!("error compiling {:?}: \n{}", vname, stuff);
                        None
                    }
                };

                let fshader = match self.create_shader(gl, &fsource, gl::FRAGMENT_SHADER) {
                    Ok(sh) => Some(sh),
                    Err(stuff) => {
                        println!("error compiling {:?}: \n{}", fname, stuff);
                        None
                    }
                };

                match (vshader, fshader) {
                    (Some(vshader), Some(fshader)) => {
                        let program = unsafe { gl.CreateProgram() };

                        unsafe {
                            gl.AttachShader(program, vshader);
                            gl.AttachShader(program, fshader);
                            gl.LinkProgram(program);
                        }

                        let mut success: gl::types::GLint = 1;
                        unsafe {
                            gl.GetProgramiv(program, gl::LINK_STATUS, &mut success);
                        }

                        if success == 0 {
                            let mut len: gl::types::GLint = 0;
                            unsafe {
                                gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                            }

                            let error: CString = unsafe {
                                CString::from_vec_unchecked(vec![b' ' as u8; len as usize])
                            };

                            unsafe {
                                gl.GetProgramInfoLog(program,
                                                     len,
                                                     ptr::null_mut(),
                                                     error.as_ptr() as *mut raw::c_char);
                            }

                            println!("error linking {:?}: \n{}",
                                     self.name,
                                     error.to_string_lossy().as_ref());

                            self.program = None;
                        } else {
                            self.program = Some(program);
                        }

                        unsafe {
                            gl.DeleteShader(vshader);
                            gl.DeleteShader(fshader);
                        }
                    }
                    (None, Some(sh)) => {
                        unsafe {
                            gl.DeleteShader(sh);
                        };
                        self.program = None;
                    }
                    (Some(sh), None) => {
                        unsafe {
                            gl.DeleteShader(sh);
                        };
                        self.program = None;
                    }
                    _ => {
                        self.program = None;
                    }
                };
            }
        }
    }
}

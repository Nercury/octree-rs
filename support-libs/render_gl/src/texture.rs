use gl::{self, Gl};
use std::os::raw;
use resources::Resources;
use image;

pub struct Texture {
    name: String,
    texture: Option<gl::types::GLuint>,
}

impl Texture {
    pub fn new(name: &str) -> Texture {
        Texture {
            name: name.to_string(),
            texture: None,
        }
    }

    fn load_image(&self, resources: &Resources) -> Option<image::DynamicImage> {
        let img_path = resources.get_file_path(&["image/", &self.name[..]].concat());
        let img = match image::open(&img_path) {
            Ok(i) => i,
            Err(e) => panic!("failed to open image {:?}, {:?}", img_path, e),
        };
        Some(img)
    }

    pub fn deinit(&mut self, gl: &Gl) {
        if let Some(ref mut id) = self.texture {
            if unsafe { gl.IsTexture(*id) } == gl::TRUE {
                unsafe { gl.DeleteTextures(1, id) };
            }
        }
        self.texture = None;
    }

    pub fn id(&self) -> Option<gl::types::GLuint> {
        self.texture
    }

    pub fn bind(&self, gl: &Gl) {
        match self.texture {
            Some(id) => unsafe {
                gl.BindTexture(gl::TEXTURE_2D, id);
            },
            None => unsafe {
                gl.BindTexture(gl::TEXTURE_2D, 0);
            },
        }
    }

    pub fn bind_at(&self, gl: &Gl, index: u32) {
        unsafe {
            gl.ActiveTexture(gl::TEXTURE0 + index);
        }

        match self.texture {
            Some(id) => unsafe {
                gl.BindTexture(gl::TEXTURE_2D, id);
            },
            None => unsafe {
                gl.BindTexture(gl::TEXTURE_2D, 0);
            },
        }
    }

    pub fn init(&mut self, gl: &Gl, resources: &Resources) {
        if let Some(image) = self.load_image(resources) {
            self.deinit(gl);

            let mut texture: gl::types::GLuint = 0;
            unsafe {
                gl.GenTextures(1, &mut texture);
            }

            if let image::ColorType::RGBA(_) = image.color() {
                let rgba = image.to_rgba();

                unsafe {
                    gl.BindTexture(gl::TEXTURE_2D, texture);
                    gl.TexImage2D(gl::TEXTURE_2D,
                                  0,
                                  gl::RGBA as gl::types::GLint,
                                  rgba.width() as i32,
                                  rgba.height() as i32,
                                  0,
                                  gl::RGBA,
                                  gl::UNSIGNED_BYTE,
                                  rgba.as_ptr() as *const raw::c_void);
                    gl.GenerateMipmap(gl::TEXTURE_2D);

                    gl.BindTexture(gl::TEXTURE_2D, 0);
                }
            } else {
                let rgb = image.to_rgb();

                unsafe {
                    gl.BindTexture(gl::TEXTURE_2D, texture);
                    gl.TexImage2D(gl::TEXTURE_2D,
                                  0,
                                  gl::RGB as gl::types::GLint,
                                  rgb.width() as i32,
                                  rgb.height() as i32,
                                  0,
                                  gl::RGB,
                                  gl::UNSIGNED_BYTE,
                                  rgb.as_ptr() as *const raw::c_void);
                    gl.GenerateMipmap(gl::TEXTURE_2D);

                    gl.BindTexture(gl::TEXTURE_2D, 0);
                }
            }

            self.texture = Some(texture);
        }
    }
}

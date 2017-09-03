#[macro_use] extern crate log;
extern crate sdl2;
extern crate gl_core_struct as gl;
extern crate window;
extern crate window_gl;
extern crate resources;
extern crate input;
extern crate input_sdl;

use sdl2::{video, Sdl};
use gl::{Glw, Gl};
use std::{mem, str};
use std::ffi;
use std::os::raw::c_char;
use resources::Resources;

pub enum GLVersion {
    Core((u8, u8)),
}

pub struct WindowOptions {
    pub gl_version: GLVersion,
    pub title: String,
    pub initial_size: (u32, u32),
    pub vsync: bool,
}

pub struct SdlWindow {
    sdl_context: Sdl,
    sdl_window: video::Window,
    _sdl_gl_context: video::GLContext,
    pub gl: Glw,
    pub size: (u32, u32),
    prev_time: u64,
}

impl SdlWindow {
    pub fn new(options: WindowOptions) -> SdlWindow {
        let sdl = sdl2::init().unwrap();
        let video_sub_sys = sdl.video().unwrap();

        // 0 for immediate updates,
        // 1 for updates synchronized with the vertical retrace,
        // -1 for late swap tearing

        video_sub_sys.gl_set_swap_interval(if options.vsync { 1 } else { -1 });

        match options.gl_version {
            GLVersion::Core((major, minor)) => {
                video_sub_sys.gl_attr().set_context_profile(video::GLProfile::Core);
                //                video_sub_sys.gl_attr().set_context_flags().debug().set();
                video_sub_sys.gl_attr().set_context_version(major, minor);
            }
        }

        video_sub_sys.gl_attr().set_accelerated_visual(true);
        video_sub_sys.gl_attr().set_double_buffer(true);

        // Enable anti-aliasing
        video_sub_sys.gl_attr().set_multisample_buffers(1);
        video_sub_sys.gl_attr().set_multisample_samples(16);

        let profile = video_sub_sys.gl_attr().context_profile();
        let (a, b) = video_sub_sys.gl_attr().context_version();
        info!("Using OpenGL {} {}.{}", match profile {
            video::GLProfile::Core => format!("Core"),
            video::GLProfile::Compatibility => format!("Compatibility"),
            video::GLProfile::GLES => format!("ES"),
            video::GLProfile::Unknown(x) => format!("{}", x),
        }, a, b);

        let (window_width, window_height) = options.initial_size;

        let window = match video_sub_sys.window(&options.title, window_width, window_height)
            .resizable()
            .opengl()
            .allow_highdpi()
            .build() {
            Ok(window) => window,
            Err(err) => panic!("failed to create window: {}", err),
        };

        let (gl_context, gl) = match window.gl_create_context() {
            Err(err) => panic!("failed to create GL context: {}", err),
            Ok(gl_context) => {
                let gl = Glw::new(Gl::load_with(|s| unsafe { mem::transmute(video_sub_sys.gl_get_proc_address(s)) }));

                unsafe {
                    let renderer = gl.GetString(gl::RENDERER);
                    let version = gl.GetString(gl::VERSION);
                    let shading = gl.GetString(gl::SHADING_LANGUAGE_VERSION);

                    info!("Available OpenGL {version} (shading language {shading}), {renderer}",
                          renderer = str::from_utf8_unchecked(ffi::CStr::from_ptr(renderer as *const c_char).to_bytes()),
                          version = str::from_utf8_unchecked(ffi::CStr::from_ptr(version as *const c_char).to_bytes()),
                          shading = str::from_utf8_unchecked(ffi::CStr::from_ptr(shading as *const c_char).to_bytes())
                    );
                }

                (gl_context, gl)
            }
        };

        SdlWindow {
            sdl_context: sdl,
            sdl_window: window,
            _sdl_gl_context: gl_context,
            size: options.initial_size,
            gl: gl,
            prev_time: 0,
        }
    }

    pub fn run<R>(&mut self, mut scene: R, resources: &Resources)
        where R: window::Render<window_gl::Info> + input::Handle<window_gl::Info>
    {
        let timer_sub_sys = self.sdl_context.timer().unwrap();
        let mut freq = timer_sub_sys.performance_frequency();

        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut window_info = window_gl::Info {
            gl: self.gl.clone(),
            window: window::Info {
                width: self.sdl_window.drawable_size().0 as i32,
                height: self.sdl_window.drawable_size().1 as i32,
            },
        };

        let input_mapper = input_sdl::SdlInputMapper::new();

        scene.init(&window_info, resources);

        self.prev_time = timer_sub_sys.performance_counter();

        'running: loop {
            let new_time = timer_sub_sys.performance_counter();
            let mut delta = (new_time - self.prev_time) as f32 / (freq as f32);
            self.prev_time = new_time;

            for event in event_pump.poll_iter() {
                use sdl2::event::Event;
                use sdl2::event::WindowEvent;

                match event {
                    Event::Quit { .. } => break 'running,
                    Event::Window {
                        win_event: WindowEvent::Resized(w, h),
                        ..
                    } => {
                        self.size = (w as u32, h as u32);
                        window_info.window.width = self.sdl_window.drawable_size().0 as i32;
                        window_info.window.height = self.sdl_window.drawable_size().1 as i32;

                        input_mapper.mouse_gone(&mut scene, &window_info);
                        scene.init(&window_info, resources);

                        freq = timer_sub_sys.performance_frequency();
                        self.prev_time = timer_sub_sys.performance_counter();
                        delta = 0.0;
                    }
                    e => {
                        input_mapper.dispatch(&mut scene, &window_info, e);
                    }
                }
            }

            scene.update(delta);
            scene.render(&window_info);

            self.sdl_window.gl_swap_window();
        }
    }
}
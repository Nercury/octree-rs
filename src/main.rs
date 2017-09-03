extern crate window;
extern crate window_sdl;
extern crate window_gl;
extern crate input;
extern crate render;
extern crate render_gl;
extern crate resources;
extern crate cgmath;
extern crate gl_core_struct as gl;
extern crate log;
extern crate env_logger;

mod cube;
mod sample_scene;

use resources::Resources;
use sample_scene::SampleScene;

fn main() {
    let mut builder = env_logger::LogBuilder::new();
    builder.filter(None, log::LogLevelFilter::Trace);
    if ::std::env::var("RUST_LOG").is_ok() {
        builder.parse(&::std::env::var("RUST_LOG").unwrap());
    }
    builder.init().unwrap();

    let resources = Resources::new()
        .with_rel_mount("/");

    let mut w = window_sdl::SdlWindow::new(window_sdl::WindowOptions {
        gl_version: window_sdl::GLVersion::Core((4, 1)),
        title: "Cube".to_string(),
        initial_size: (1024, 720),
        vsync: false,
    });
    w.run(
        SampleScene::new(
            cube::Cube::new()
        ),
        &resources
    );
}
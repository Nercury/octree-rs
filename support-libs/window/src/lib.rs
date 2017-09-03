extern crate resources;

#[derive(Debug, Copy, Clone)]
pub struct Info {
    pub width: i32,
    pub height: i32,
}

pub trait Render<W> {
    fn init(&mut self, _window: &W, _resources: &resources::Resources) {}
    fn update(&mut self, _delta: f32) {}
    fn render(&self, _window: &W) {}
}
use input;
use window;
use window_gl;
use gl;
use resources::Resources;

pub struct SampleScene<T> {
    pub obj: T,
}

impl<T> SampleScene<T> {
    pub fn new(obj: T) -> SampleScene<T> {
        SampleScene::<T> { obj: obj }
    }
}

impl<T: window::Render<window_gl::Info>> window::Render<window_gl::Info> for SampleScene<T> {
    fn init(&mut self, window: &window_gl::Info, resources: &Resources) {
        let gl = &window.gl;
        unsafe {
            gl.Viewport(0, 0, window.window.width, window.window.height);
            gl.ClearColor(0.02, 0.02, 0.02, 1.0);
        }
        self.obj.init(window, resources);
    }

    fn update(&mut self, delta: f32) {
        self.obj.update(delta);
    }

    fn render(&self, window: &window_gl::Info) {
        let gl = &window.gl;
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.obj.render(window);
    }
}

impl<T: input::Handle<window_gl::Info>> input::Handle<window_gl::Info> for SampleScene<T> {
    fn handle(&mut self, window: &window_gl::Info, event: input::InputEvent) {
        self.obj.handle(window, event);
    }
}
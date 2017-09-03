use gl::Glw;

#[derive(Debug, Copy, Clone)]
pub struct NextFrame {
    pub vbo_size: usize,
    pub ebo_size: usize,
}

pub trait VertexSource {
    fn vao_stride(&self) -> usize;
    fn vao_init(&mut self, gl: &Glw);
    fn update(&mut self, delta: f32) -> Option<NextFrame>;
    fn fill(&self, previous_data: usize, vbo_slice: &mut [u8], ebo_slice: &mut [u8]) -> usize;
}

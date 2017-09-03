extern crate sdl2;

pub use sdl2::event::Event;

pub trait Handle<W> {
    fn handle(&mut self, _window: &W, _event: InputEvent) {}
}

#[derive(Clone, PartialEq, Debug)]
pub enum InputEvent {
    Mouse(MouseEvent),
    Other(Event)
}

#[derive(Clone, PartialEq, Debug)]
pub enum MouseEvent {
    Gone,
    At(Event),
}
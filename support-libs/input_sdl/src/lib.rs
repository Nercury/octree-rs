extern crate input;
extern crate sdl2;

use sdl2::event::Event;

pub struct SdlInputMapper {}

impl SdlInputMapper {
    pub fn new() -> SdlInputMapper {
        SdlInputMapper {}
    }

    #[inline]
    pub fn mouse_gone<W, H: input::Handle<W>>(&self, handler: &mut H, window_info: &W) {
        handler.handle(&window_info, input::InputEvent::Mouse(input::MouseEvent::Gone));
    }

    #[inline]
    pub fn dispatch<W, H: input::Handle<W>>(&self, handler: &mut H, window_info: &W, e: Event) {
        match e {
            Event::MouseMotion { .. } => {
                handler.handle(&window_info, input::InputEvent::Mouse(input::MouseEvent::At(e)));
            }
            other => handler.handle(&window_info, input::InputEvent::Other(other)),
        }
    }
}
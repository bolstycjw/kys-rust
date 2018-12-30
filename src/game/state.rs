use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston_window::*;

pub trait State {
    fn box_clone(&self) -> Box<dyn State>;

    fn handle_events(&mut self, event: &Event);

    fn next_state(&self) -> Option<Box<dyn State>>;

    fn on_load(&mut self, window: &mut PistonWindow);

    fn render(&self, c: &Context, g: &mut GfxGraphics<Resources, CommandBuffer>);
}

impl Clone for Box<dyn State> {
    fn clone(&self) -> Box<dyn State> {
        self.box_clone()
    }
}

use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston_window::*;

pub trait State {
    // fn handle_events(&mut self);

    fn on_load(&mut self, window: &mut PistonWindow);

    fn render(&self, c: &Context, g: &mut GfxGraphics<Resources, CommandBuffer>);
}

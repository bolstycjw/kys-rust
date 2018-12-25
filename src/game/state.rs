use opengl_graphics::*;
use piston_window::*;

pub trait State {
    // fn handle_events(&mut self);
    fn render(&self, c: &Context, g: &mut GlGraphics);
}

use piston_window::*;

pub trait State {
    fn box_clone(&self) -> Box<dyn State>;

    fn handle_events(&mut self, event: &Event);

    fn next_state(&self) -> Option<Box<dyn State>>;

    fn on_load(&mut self, w: &mut PistonWindow);

    fn render(&mut self, e: &Event, w: &mut PistonWindow);
}

impl Clone for Box<dyn State> {
    fn clone(&self) -> Box<dyn State> {
        self.box_clone()
    }
}

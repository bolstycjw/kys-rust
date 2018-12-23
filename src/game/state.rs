pub trait EventHandler {
    fn handle_events(&self) {}
}

pub trait Renderable {
    fn render(&self) {}
}

pub enum State {
    Scene,
}

impl EventHandler for State {}
impl Renderable for State {}

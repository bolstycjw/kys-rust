pub mod scene;
pub mod state;

use self::scene::Scene;
use self::state::{EventHandler, Renderable, State};
use crate::context::Context;

pub struct Game<'a> {
    context: Context<'a>,
    scenes: Vec<Scene>,
    cur_state: State,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let context = Context::init();

        Game { context }
    }

    pub fn run(&self) {
        self.cur_state.handle_events();
        self.cur_state.render();
    }
}

pub mod scene;
pub mod state;
pub mod tile;
pub mod title;

use piston_window::*;
use std::boxed::Box;

use self::state::State;
use self::title::Title;

pub struct Game {
    state: Box<dyn State>,
}

impl Game {
    pub fn new() -> Self {
        {
            Self {
                state: Box::new(Title::new()),
            }
        }
    }

    pub fn change_state(&mut self, maybe_next_state: Option<Box<dyn State>>) {
        if let Some(next_state) = maybe_next_state {
            self.state = next_state;
        };
    }

    pub fn run(&mut self, window: &mut PistonWindow) {
        self.state.on_load(window);
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(window) {
            if let Some(_r) = e.render_args() {
                window.draw_2d(&e, |_c, g| {
                    clear([0.0; 4], g);
                });
                self.state.render(&e, window);
            };

            if let Some(_args) = e.button_args() {
                self.state.handle_events(&e);
            };

            self.change_state(self.state.next_state());
        }
    }
}

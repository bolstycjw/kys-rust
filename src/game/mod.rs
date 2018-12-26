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

    pub fn change_state(&mut self, next_state: Box<dyn State>) {
        self.state = next_state;
    }

    pub fn run(&mut self, window: &mut PistonWindow) {
        let opengl = OpenGL::V3_2;
        let mut window: PistonWindow = WindowSettings::new("piston: image", [300, 300])
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();

        window.set_lazy(true);

        self.state.on_load(&mut window);
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                self.state.render(&c, g);
            });
        }
    }
}

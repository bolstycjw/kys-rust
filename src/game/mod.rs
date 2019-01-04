// pub mod scene;
// pub mod tile;
pub mod title;

use graphics::Context;
use opengl_graphics::GlGraphics;

use self::title::Title;

pub struct Game {
    title: Title,
}

impl Game {
    pub fn new() -> Self {
        {
            Self {
                title: Title::new(),
            }
        }
    }

    pub fn render(&mut self, c: &Context, g: &mut GlGraphics) {
        self.title.render(c, g);
    }
}

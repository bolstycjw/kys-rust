use graphics::{image, Context, ImageSize, Transformed};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};

use crate::config::*;

pub struct Title {
    title: Texture,
}

impl Title {
    pub fn new() -> Self {
        let resource = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("resource")
            .unwrap();
        let title = resource.join("open.png");
        let title =
            Texture::from_path(title, &TextureSettings::new()).expect("Could not load title");
        Self { title: title }
    }

    pub fn render(&mut self, c: &Context, g: &mut GlGraphics) {
        let (width, height) = self.title.get_size();
        let sx = SCREEN_WIDTH as f64 / width as f64;
        let sy = SCREEN_HEIGHT as f64 / height as f64;
        image(&self.title, c.transform.scale(sx, sy), g);
    }
}

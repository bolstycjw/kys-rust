extern crate find_folder;
extern crate piston_window;

mod config;
mod engine;
mod game;

use piston_window::*;

pub fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let rust_logo = assets.join("rust.png");
    let rust_logo: G2dTexture = Texture::from_path(
        &mut window.factory,
        &rust_logo,
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();
    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            image(&rust_logo, c.transform, g);
        });
    }
}

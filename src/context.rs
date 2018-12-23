use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

pub struct Context {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl<'a> Context {
    pub fn init() -> Context {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("kys rust", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        Context {
            sdl_context,
            canvas,
            texture_creator,
        }
    }
}

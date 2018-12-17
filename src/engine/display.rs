use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub mod tile;

pub struct DisplayContext(Canvas<Window>);

pub fn init(sdl_context: Sdl) -> DisplayContext {
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("kys rust", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();
    DisplayContext(canvas)
}

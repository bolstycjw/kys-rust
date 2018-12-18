pub mod display;
pub mod math;

use self::display::DisplayContext;

pub struct Context<'a> {
    pub disp_ctx: DisplayContext<'a>,
}

impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        let sdl_ctx = sdl2::init().unwrap();
        Context {
            disp_ctx: DisplayContext::new(sdl_ctx),
        }
    }
}

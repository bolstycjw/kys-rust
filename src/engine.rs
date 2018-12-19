pub mod display;
pub mod math;

use self::display::DisplayContext;

pub struct Context<'a> {
    pub display_context: DisplayContext<'a>,
}

impl<'a> Context<'a> {
    pub fn init() -> Context<'a> {
        let sdl_ctx = sdl2::init().unwrap();
        Context {
            display_context: DisplayContext::new(sdl_ctx),
        }
    }
}

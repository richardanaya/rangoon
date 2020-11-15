pub use hecs::*;
use web::*;
pub use web::random;
pub use web::{Mutex,MutexGuard};
pub use web::CanvasContext;
pub use web::Canvas2dApi;


pub struct Graphics {
    pub ctx: CanvasContext,
    pub width: i32,
    pub height: i32,
}

pub trait Game {
    fn new() -> Self;
    fn instance() -> MutexGuard<'static, Self> ;
    fn build_graphics() -> Graphics {
        let screen = get_element_by_id("screen");
        let width: f64 = get_property(&screen, "width");
        let height: f64 = get_property(&screen, "height");
        let ctx = CanvasContext::from_canvas_element(&screen);
        Graphics {
            ctx,
            width:width as i32,
            height: height as i32,
        }
    }
    fn start() where Self: 'static{
        add_event_listener(DOM_BODY, "keydown", |event| {
            let key_down_event = KeyDownEvent::from_event(event);
            let key_code = key_down_event.key_code();
            Self::instance().key_down(key_code);
        });
        request_animation_loop(|delta| Self::instance().run(delta));
    }
    fn key_down(&mut self, _key_code: u32){}
    fn run(&mut self, _delta: f64){}
}
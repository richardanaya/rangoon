use rangoon::*;

struct Simplest {
    gfx: Graphics,
}

impl Game for Simplest {
    fn new() -> Simplest {
        Simplest {
            gfx: Simplest::build_graphics(),
        }
    }

    fn instance() -> MutexGuard<'static, Simplest> {
        lazy_static::lazy_static! {
            static ref SINGLETON: Mutex<Simplest> = {
                Mutex::new(Simplest::new())
            };
        }
        SINGLETON.lock()
    }

    fn run(&mut self, _delta: f64) {
        self.gfx.ctx.set_fill_color("red");
        self.gfx.ctx.fill_rect(50, 50, 50, 50);
    }
}

#[no_mangle]
pub fn main() {
    Simplest::start();
}

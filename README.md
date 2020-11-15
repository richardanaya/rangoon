# rangoon

A simple game engine for WebAssembly.

## Example

```rust
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

    fn key_down(&mut self, key_code: u32){
      ...
    }

    fn run(&mut self, delta: f64){
      ...
    }
}

#[no_mangle]
pub fn main() {
    Simplest::start();
}
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `rangoon` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

use rangoon::*;

struct Snake {
    time: i32,
    gfx: Graphics,
    direction: Direction,
    width: i32,
    height: i32,
    world: World,
    head: Entity,
    made_move: bool,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// ECS components
struct SnakeHead(i32);
struct SnakeBody(i32);
struct Food;
struct Position(i32, i32);
struct Color(String);

const MAP_WIDTH: i32 = 30;
const MAP_HEIGHT: i32 = 30;
const ITERATION_TIME: i32 = 100;

impl Game for Snake {
    fn new() -> Snake {
        // create snake
        let mut world = World::new();
        let head = world.spawn((
            SnakeHead(1),
            Color("green".to_string()),
            Position(MAP_WIDTH / 2, MAP_HEIGHT / 2),
        ));
        let mut g = Snake {
            time: 0,
            gfx: Self::build_graphics(),
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            direction: Direction::Down,
            head,
            world,
            made_move: false,
        };
        g.reset();
        g
    }

    fn instance() -> MutexGuard<'static, Snake> {
        lazy_static::lazy_static! {
            static ref SINGLETON: Mutex<Snake> = {
                Mutex::new(Snake::new())
            };
        }
        SINGLETON.lock()
    }

    fn key_down(&mut self, key_code: u32) {
        if self.made_move {
            return;
        }
        self.made_move = true;
        match key_code {
            87 | 38 => {
                if let Direction::Down = self.direction {
                } else {
                    self.direction = Direction::Up
                }
            }
            68 | 39 => {
                if let Direction::Left = self.direction {
                } else {
                    self.direction = Direction::Right
                }
            }
            83 | 40 => {
                if let Direction::Up = self.direction {
                } else {
                    self.direction = Direction::Down
                }
            }
            65 | 37 => {
                if let Direction::Right = self.direction {
                } else {
                    self.direction = Direction::Left
                }
            }
            _ => (),
        };
    }

    fn run(&mut self, delta: f64) {
        self.time += delta as i32;
        if self.time > ITERATION_TIME {
            self.time %= ITERATION_TIME;
            self.move_snake_system();
            self.eat_system();
        }
        self.render_system();
        self.made_move = false;
    }
}

impl Snake {
    fn reset(&mut self) {
        self.gfx
            .ctx
            .clear_rect(0, 0, self.gfx.width, self.gfx.height);
        self.world.clear();
        self.head = self.world.spawn((
            SnakeHead(1),
            Color("green".to_string()),
            Position(MAP_WIDTH / 2, MAP_HEIGHT / 2),
        ));
        self.spawn_food();
    }

    fn spawn_food(&mut self) {
        // create initial food
        self.world.spawn((
            Food,
            Color("red".to_string()),
            Position(
                (random() * MAP_WIDTH as f64) as i32,
                (random() * MAP_HEIGHT as f64) as i32,
            ),
        ));
    }

    fn move_snake_system(&mut self) -> Result<(), ComponentError> {
        let (last_head_pos, next_head_pos) = {
            let mut pos = self.world.get_mut::<Position>(self.head)?;
            let p = Position(pos.0, pos.1);
            match self.direction {
                Direction::Up => pos.1 -= 1,
                Direction::Right => pos.0 += 1,
                Direction::Down => pos.1 += 1,
                Direction::Left => pos.0 -= 1,
            }
            (p, Position(pos.0, pos.1))
        };
        let mut body_to_remove = vec![];
        let mut bit_tail = false;
        if next_head_pos.0 < 0
            || next_head_pos.1 < 0
            || next_head_pos.0 > self.width
            || next_head_pos.1 > self.height
        {
            self.reset();
            return Ok(());
        }
        for (id, (body, pos)) in &mut self.world.query::<(&mut SnakeBody, &Position)>() {
            body.0 -= 1;
            if body.0 == 0 {
                body_to_remove.push(id);
            } else {
                if pos.0 == next_head_pos.0 && pos.1 == next_head_pos.1 {
                    bit_tail = true;
                    break;
                }
            }
        }
        if bit_tail {
            self.reset();
            return Ok(());
        }
        for b in body_to_remove.into_iter() {
            self.world.despawn(b)?;
        }
        let snake_level = self.world.get::<SnakeHead>(self.head)?.0;
        self.world.spawn((
            SnakeBody(snake_level),
            Color("forestgreen".to_string()),
            last_head_pos,
        ));
        Ok(())
    }

    fn render_system(&self) {
        self.gfx
            .ctx
            .clear_rect(0, 0, self.gfx.width, self.gfx.height);
        for (_id, (pos, color)) in &mut self.world.query::<(&Position, &Color)>() {
            self.gfx.ctx.set_fill_color(&color.0);
            self.gfx.ctx.fill_rect(
                pos.0 * (self.gfx.width / MAP_WIDTH),
                pos.1 * (self.gfx.height / MAP_HEIGHT),
                self.gfx.width / MAP_WIDTH,
                self.gfx.height / MAP_HEIGHT,
            );
        }
    }

    fn eat_system(&mut self) -> Result<(), ComponentError> {
        let (head_x, head_y) = {
            let p = self.world.get::<Position>(self.head)?;
            (p.0, p.1)
        };
        let mut food_to_remove = None;
        for (id, (_, pos)) in &mut self.world.query::<(&Food, &Position)>() {
            if pos.0 == head_x && pos.1 == head_y {
                food_to_remove = Some(id);
                break;
            }
        }
        if let Some(id) = food_to_remove {
            {
                self.world.despawn(id)?;
            }
            {
                let mut head = self.world.get_mut::<SnakeHead>(self.head)?;
                head.0 += 1;
            }
            self.spawn_food();
        }
        Ok(())
    }
}

#[no_mangle]
pub fn main() {
    Snake::start();
}

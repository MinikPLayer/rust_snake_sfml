use std::process::exit;
use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture, Transformable};
use sfml::system::{Clock, Vector2, Vector2f, Vector2i};
use sfml::window::{Event, Key, Style};

extern crate msgbox;
use msgbox::IconType;

const SEGMENT_SIZE: u32 = 50;
const BOARD_SIZE: u32 = 20;

#[derive(Copy, Clone)]
enum Directions {
    STOP = 0,
    RIGHT = 1,
    LEFT = -1,
    UP = 2,
    DOWN = -2
}

struct Snake {
    positions: Vec<Vector2i>,
    positions_to_add: i32,
    dir: Directions,
    dir_queue: Vec<Directions>,
    apple_pos: Vector2i,
    score: i32,
    lives: i8
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            positions: vec![Vector2::new(0, 0), Vector2::new(1, 0), Vector2::new(2, 0), Vector2::new(3, 0)],
            positions_to_add: 0,
            dir: Directions::RIGHT,
            dir_queue: Vec::new(),
            apple_pos: Vector2i::new(-1, -1),
            score: 0,
            lives: 3
        }
    }
}

fn get_random_vector2i(min: i32, max: i32) -> Vector2i {
    Vector2i::new(
        rand::random::<i32>().abs() % (max - min) + min,
        rand::random::<i32>().abs() % (max - min) + min
    )
}

fn game_over() {
    msgbox::create("Game status", "GAME OVER!", IconType::Info).expect("Cannot create message box");
    exit(0);
}

fn key_pressed(k: Key, data: &mut Snake) {
    let new_dir = match k {
        Key::Right => Directions::RIGHT,
        Key::Left => Directions::LEFT,
        Key::Up => Directions::UP,
        Key::Down => Directions::DOWN,
        _ => Directions::STOP,
    };

    if !matches!(new_dir, Directions::STOP) && data.dir_queue.len() < 4 {
        data.dir_queue.push(new_dir);
    }
}

fn snake_clamp(x: i32, min: i32, max: i32) -> i32 {
    if x < min {
        return max + x - min;
    }
    if x >= max {
        return x - max + min;
    }
    return x;
}

fn spawn_apple(snake_data: &mut Snake) {
    let mut good: bool = false;
    let mut pos: Vector2i = Vector2i::new(0, 0);
    while !good {
        pos = get_random_vector2i(0, BOARD_SIZE as i32);
        good = true;
        for p in &snake_data.positions {
            if pos == *p {
                good = false;
                break;
            }
        }
    }

    snake_data.apple_pos = pos;
}

fn move_snake(snake_data: &mut Snake) {
    if snake_data.dir_queue.len() > 0 {
        let dir = snake_data.dir_queue[0];
        if dir as i32 != -(snake_data.dir as i32) {
            snake_data.dir = snake_data.dir_queue[0];
        }
        snake_data.dir_queue.remove(0);
    }

    let move_dir = match snake_data.dir {
        Directions::RIGHT => Vector2i::new(1, 0),
        Directions::LEFT => Vector2i::new(-1, 0),
        Directions::UP => Vector2i::new(0, -1),
        Directions::DOWN => Vector2i::new(0, 1),
        Directions::STOP => Vector2i::new(0, 0)
    };

    let last_pos = snake_data.positions[snake_data.positions.len() - 1];
    if snake_data.positions_to_add > 0 {
        snake_data.positions_to_add -= 1;
    }
    else {
        snake_data.positions.remove(0);
    }


    let mut new_pos = last_pos + move_dir;
    new_pos.x = snake_clamp(new_pos.x, 0, BOARD_SIZE as i32);
    new_pos.y = snake_clamp(new_pos.y, 0, BOARD_SIZE as i32);

    for p in &snake_data.positions {
        if new_pos == *p {
            game_over();
        }
    }
    snake_data.positions.push(new_pos);

    if new_pos == snake_data.apple_pos {
        snake_data.score += 1;
        snake_data.positions_to_add += 1;
        spawn_apple(snake_data);
    }
}

fn draw(window: &mut RenderWindow, snake_data: &mut Snake) {
    let mut txt = Texture::new().expect("Cannot create texture");
    if !txt.create(SEGMENT_SIZE, SEGMENT_SIZE) {
        panic!("Cannot create texture");
    }

    unsafe {
        let mut pixels = [0u8; SEGMENT_SIZE as usize * SEGMENT_SIZE as usize * 4];
        for p in &mut pixels {
            *p = 255;
        }
        txt.update_from_pixels(&pixels, SEGMENT_SIZE, SEGMENT_SIZE, 0, 0);
    }

    let mut sprite = Sprite::new();
    sprite.set_texture(&txt, false);

    for pos in &snake_data.positions {
        sprite.set_position(Vector2f::new((pos.x * SEGMENT_SIZE as i32) as f32, (pos.y * SEGMENT_SIZE as i32) as f32));
        window.draw(&sprite);
    }

    // Draw apple
    sprite.set_color(Color::RED);
    sprite.set_position(Vector2f::new((snake_data.apple_pos.x * SEGMENT_SIZE as i32) as f32, (snake_data.apple_pos.y * SEGMENT_SIZE as i32) as f32));
    window.draw(&sprite);
}

fn main() {
    let mut window = RenderWindow::new((BOARD_SIZE * SEGMENT_SIZE, BOARD_SIZE * SEGMENT_SIZE),
        "Rust Snake",
        Style::DEFAULT,
        &Default::default()
    );
    let mut clock = Clock::start();
    let mut snake_data: Snake = Snake::default();
    spawn_apple(&mut snake_data);

    let mut snake_timer: f32 = 0f32;
    let mut snake_move_interval: f32 = 150f32;
    while window.is_open() {
        let delta_time = clock.restart();

        snake_timer += (delta_time.as_microseconds() as f64 / 1000f64) as f32;
        // Event processing
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed {code, .. } => key_pressed(code, &mut snake_data),
                _ => {}
            }
        }

        if snake_timer >= snake_move_interval {
            snake_timer -= snake_move_interval;
            move_snake(&mut snake_data);
        }

        window.clear(Color::BLACK);

        draw(&mut window,&mut snake_data);
        window.display();
    }
}

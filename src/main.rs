extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use rand::Rng;

enum SpawnDirection {
    South,
    North,
    West,
    East,
}

enum Direction {
    Right,
    Left,
    Straight,
}

struct Car {
    id: u32,
    direction: Direction,
    spawn_direction: SpawnDirection,
    x: i32,
    y: i32,
    color: Color,
}

impl Car {
    fn update(&mut self, speed: i32, window_width: i32, window_height: i32, cw: i32, ch: i32, gap: i32) {
        match self.spawn_direction {
            SpawnDirection::North => {
                match self.direction {
                    Direction::Right => {
                        if self.y >= ch {
                            self.x += speed;
                            self.y = ch;
                        } else {
                            self.y += speed;
                        }
                    },
                    Direction::Left => {
                        if self.y >= ch - gap {
                            self.x -= speed;
                            self.y = ch - gap;
                        } else {
                            self.y += speed;
                        }
                    },
                    Direction::Straight => {
                        self.y += speed;
                    }
                }
            },
            SpawnDirection::South => {
                match self.direction {
                    Direction::Right => {
                        if self.y <= ch - gap {
                            self.x -= speed;
                            self.y = ch - gap;
                        } else {
                            self.y -= speed;
                        }
                    },
                    Direction::Left => {
                        if self.y <= ch {
                            self.x += speed;
                            self.y = ch;
                        } else {
                            self.y -= speed;
                        }
                    },
                    Direction::Straight => {
                        self.y -= speed;
                    }
                }
            },
            SpawnDirection::West => {
                match self.direction {
                    Direction::Left => {
                        if self.x >= cw - gap {
                            self.y += speed;
                            self.x = cw - gap;
                        } else {
                            self.x += speed;
                        }
                    },
                    Direction::Right => {
                        if self.x >= cw {
                            self.y -= speed;
                            self.x = cw;
                        } else {
                            self.x += speed;
                        }
                    },
                    Direction::Straight => {
                        self.x += speed;
                    }
                }
            },
            SpawnDirection::East => {
                match self.direction {
                    Direction::Left => {
                        if self.x <= cw {
                            self.y -= speed;
                            self.x = cw;
                        } else {
                            self.x -= speed;
                        }
                    },
                    Direction::Right => {
                        if self.x <= cw - gap {
                            self.y += speed;
                            self.x = cw - gap;
                        } else {
                            self.x -= speed;
                        }
                    },
                    Direction::Straight => {
                        self.x -= speed;
                    }
                }
            }
        }
    }

    fn is_off_screen(&self, window_width: i32, window_height: i32, car_size: i32) -> bool {
        self.x < -car_size || 
        self.x > window_width || 
        self.y < -car_size || 
        self.y > window_height
    }
}

fn spawn_car(
    id: u32,
    window_width: i32,
    window_height: i32,
    cw: i32,
    ch: i32,
    gap: i32,
    car_size: i32,
    spawn_direction: Option<SpawnDirection>
) -> Car {
    let mut rng = rand::thread_rng();

    let spawn_direction = match spawn_direction {
        Some(dir) => dir,
        None =>
            match rng.gen_range(0..4) {
                0 => SpawnDirection::North,
                1 => SpawnDirection::South,
                2 => SpawnDirection::West,
                3 => SpawnDirection::East,
                _ => unreachable!(),
            }
    };

    let direction = match rng.gen_range(0..3) {
        0 => Direction::Right,
        1 => Direction::Left,
        2 => Direction::Straight,
        _ => unreachable!(),
    };

    let (x, y) = match spawn_direction {
        SpawnDirection::North => (cw - gap, -car_size),
        SpawnDirection::South => (cw, window_height),
        SpawnDirection::West => (-car_size, ch),
        SpawnDirection::East => (window_width, ch - gap),
    };

    let color = match direction {
        Direction::Left => Color::RGB(0, 0, 255),
        Direction::Right => Color::RGB(255, 0, 0),
        Direction::Straight => Color::RGB(0, 255, 0),
    };

    Car {
        id,
        direction,
        spawn_direction,
        x,
        y,
        color,
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width: i32 = 800;
    let window_height: i32 = 600;
    let cw: i32 = window_width / 2;
    let ch: i32 = window_height / 2;
    let gap: i32 = 50;
    let rect_size: i32 = 45;
    let car_size: i32 = 50;
    let rect_gap: i32 = 10;

    let mut cars: Vec<Car> = Vec::new();
    let mut next_id: u32 = 1;

    
    let window = video_subsystem
    .window(
        "Road Intersection",
        window_width.try_into().unwrap(),
        window_height.try_into().unwrap()
    )
    .build()
    .unwrap();
    
    let speed = 3;
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyUp { keycode: Some(key), .. } => {
                    let spawn_direction = match key {
                        Keycode::Up => Some(SpawnDirection::North),
                        Keycode::Down => Some(SpawnDirection::South),
                        Keycode::Left => Some(SpawnDirection::West),
                        Keycode::Right => Some(SpawnDirection::East),
                        Keycode::R => None,
                        _ => None,
                    };

                    if spawn_direction.is_some() || key == Keycode::R {
                        let car = spawn_car(
                            next_id,
                            window_width,
                            window_height,
                            cw,
                            ch,
                            gap,
                            car_size,
                            spawn_direction
                        );
                        cars.push(car);
                        next_id += 1;
                    }
                }
                _ => {}
            }
        }

        for car in &mut cars {
            car.update(speed, window_width, window_height, cw, ch, gap);
        }

        cars.retain(|car| !car.is_off_screen(window_width, window_height, car_size));

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_line((0, ch + gap), (window_width, ch + gap)).unwrap();
        canvas.draw_line((0, ch - gap), (window_width, ch - gap)).unwrap();
        canvas.draw_line((cw + gap, 0), (cw + gap, window_height)).unwrap();
        canvas.draw_line((cw - gap, 0), (cw - gap, window_height)).unwrap();

        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        canvas.set_draw_color(Color::RGBA(255, 255, 255, 80));
        canvas.draw_line((0, ch), (window_width, ch)).unwrap();
        canvas.draw_line((cw, 0), (cw, window_height)).unwrap();

        canvas.set_draw_color(Color::RGB(128, 128, 128));
        canvas
            .draw_rect(
                Rect::new(
                    cw - gap - rect_gap - rect_size,
                    ch - gap - rect_gap - rect_size,
                    rect_size as u32,
                    rect_size as u32
                )
            )
            .unwrap();
        canvas
            .draw_rect(
                Rect::new(
                    cw + gap + rect_gap,
                    ch - gap - rect_gap - rect_size,
                    rect_size as u32,
                    rect_size as u32
                )
            )
            .unwrap();
        canvas
            .draw_rect(
                Rect::new(
                    cw - gap - rect_gap - rect_size,
                    ch + gap + rect_gap,
                    rect_size as u32,
                    rect_size as u32
                )
            )
            .unwrap();
        canvas
            .draw_rect(
                Rect::new(
                    cw + gap + rect_gap,
                    ch + gap + rect_gap,
                    rect_size as u32,
                    rect_size as u32
                )
            )
            .unwrap();

        for car in &cars {
            canvas.set_draw_color(car.color);
            canvas.fill_rect(Rect::new(car.x, car.y, car_size as u32, car_size as u32)).unwrap();
        }
        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }
}
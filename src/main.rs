use macroquad::{prelude::*, rand::ChooseRandom};
use std::fs;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
    collided: bool,
}

impl Shape {
    // Assumes self is a circle and other a rect
    fn collides_with(&self, other: &Self) -> bool {
        self.circle().overlaps_rect(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }

    fn circle(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            r: self.size / 2.0,
        }
    }
}

#[cfg(target_family = "wasm")]
fn is_webassembly() -> bool {
    true
}

#[cfg(not(target_family = "wasm"))]
fn is_webassembly() -> bool {
    false
}

#[macroquad::main("My Game")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    const MOVEMENT_SPEED: f32 = 200.0;
    let colors = [GREEN, BLUE, RED];
    let mut squares = vec![];
    let mut bullets = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: YELLOW,
        collided: false,
    };

    let mut gameover = false;
    let mut score: u32 = 0;
    let mut high_score: u32;

    if is_webassembly() {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        high_score = storage
            .get("highscore")
            .map_or(Ok(0), |i| i.parse())
            .unwrap_or(0);
    } else {
        high_score = fs::read_to_string("highscore.dat")
            .map_or(Ok(0), |i| i.parse())
            .unwrap_or(0);
    }

    loop {
        clear_background(DARKPURPLE);
        let delta_time = get_frame_time();

        if !gameover {
            if is_key_down(KeyCode::Right) {
                circle.x += MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Left) {
                circle.x -= MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Down) {
                circle.y += MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Up) {
                circle.y -= MOVEMENT_SPEED * delta_time;
            }
            if is_key_pressed(KeyCode::Space) {
                bullets.push(Shape {
                    x: circle.x,
                    y: circle.y,
                    speed: circle.speed * 2.0,
                    size: 5.0,
                    collided: false,
                    color: RED,
                });
            }
        }
        circle.x = clamp(circle.x, 0.0, screen_width());
        circle.y = clamp(circle.y, 0.0, screen_height());

        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 2.0, bullet.color);
        }
        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);

        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
                color: *colors.choose().unwrap(),
                collided: false,
            });
        }

        if !gameover {
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }
        }

        squares.retain(|square| square.y < screen_height() + square.size);
        bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

        squares.retain(|square| !square.collided);
        bullets.retain(|bullet| !bullet.collided);

        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color,
            );
        }

        if squares.iter().any(|square| circle.collides_with(square)) {
            if score == high_score {
                if is_webassembly() {
                    let storage = &mut quad_storage::STORAGE.lock().unwrap();
                    storage.set("highscore", high_score.to_string().as_str());
                } else {
                    fs::write("highscore.dat", high_score.to_string()).ok();
                }
            }
            gameover = true;
        }

        for square in squares.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.collides_with(square) {
                    bullet.collided = true;
                    square.collided = true;
                    score += square.size.round() as u32;
                    high_score = high_score.max(score);
                }
            }
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            bullets.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            score = 0;
            gameover = false;
        }

        draw_text(
            format!("Score: {}", score).as_str(),
            10.0,
            35.0,
            25.0,
            WHITE,
        );

        let high_score_text = format!("High score: {}", high_score);
        let text_dimensions = measure_text(high_score_text.as_str(), None, 25, 1.0);
        draw_text(
            high_score_text.as_str(),
            screen_width() - text_dimensions.width - 10.0,
            35.0,
            25.0,
            WHITE,
        );

        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }

        next_frame().await
    }
}

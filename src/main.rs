use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    loop {
        clear_background(GREEN);
        next_frame().await
    }
}

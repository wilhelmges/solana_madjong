use macroquad::prelude::*;

#[macroquad::main("Mahjong")]
async fn main() {
    loop {
        clear_background(WHITE);

        draw_text("Hello, Mahjong!", 100.0, 100.0, 40.0, BLACK);

        next_frame().await;
    }
}

mod background_texture_atlas;
mod base;
mod base_texture_atlas;
mod bird_texture_atlas;
mod components;
mod game_state;
mod music_player;
mod number_texture_atlas;
mod pipe_texture_atlas;
mod pipes;
mod player;
mod sound_effects;
mod world;

use crate::game_state::GameState;
use macroquad::prelude::*;
pub const SCREEN_WIDTH: f32 = 320.0;
pub const SCREEN_HEIGHT: f32 = 568.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tiny Flappy Bird".to_owned(),
        window_width: 320,
        window_height: 568,
        window_resizable: false,
        ..Default::default()
    }
}

fn draw_loading_screen() {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    // Draw "Loading assets..." text
    let text = "Loading assets...";
    let font_size = 30.0;
    let text_dims = measure_text(text, None, font_size as u16, 1.0);
    let text_x = center_x - text_dims.width / 2.0;
    let text_y = center_y;
    draw_text(text, text_x, text_y, font_size, WHITE);

    // Draw static spinner dots
    let spinner_radius = 20.0;
    let spinner_y = center_y + 40.0;

    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::PI / 4.0;
        let x = center_x + angle.cos() * spinner_radius;
        let y = spinner_y + angle.sin() * spinner_radius;
        draw_circle(x, y, 3.0, WHITE);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    clear_background(BLACK);
    draw_loading_screen();
    next_frame().await;
    let message = load_texture("assets/sprites/message.png")
        .await
        .expect("Failed to load message");
    message.set_filter(FilterMode::Nearest);

    let mut game_state = GameState::new().await;

    loop {
        clear_background(BLACK);
        game_state = game_state.update();
        game_state.draw(&message);
        next_frame().await
    }
}

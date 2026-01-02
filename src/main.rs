mod music_player;
mod game_state;
mod player;
mod components;
mod bird_texture_atlas;
mod world;
mod background_texture_atlas;
mod pipe_texture_atlas;
mod base_texture_atlas;
mod pipes;
mod number_texture_atlas;
mod base;
mod sound_effects;

use macroquad::prelude::*;
use crate::game_state::GameState;
use crate::music_player::MusicPlayer;
use crate::player::Player;
use crate::sound_effects::SoundEffects;
use crate::world::World;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tiny Flappy Bird".to_owned(),
        window_width: 320,
        window_height: 568,
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
    // Show loading screen
    clear_background(BLACK);
    draw_loading_screen();
    next_frame().await;

    // Load all assets sequentially
    let start_time = get_time();
    println!("[LOADING] Starting asset loading at {:.2}s", start_time);

    let t = get_time();
    println!("[LOADING] Loading background texture...");
    let background = load_texture("assets/sprites/background-day.png")
        .await
        .expect("Failed to load background");
    println!("[LOADING] Background loaded in {:.2}s", get_time() - t);

    let t = get_time();
    println!("[LOADING] Loading message texture...");
    let message = load_texture("assets/sprites/message.png")
        .await
        .expect("Failed to load message");
    println!("[LOADING] Message loaded in {:.2}s", get_time() - t);

    let t = get_time();
    println!("[LOADING] Loading music...");
    let music_player = MusicPlayer::new("assets/music", 2.0)
        .await
        .expect("Failed to load music");
    println!("[LOADING] Music loaded in {:.2}s", get_time() - t);

    println!("[LOADING] Total loading time: {:.2}s", get_time() - start_time);

    // Set texture filter to nearest for pixel art
    background.set_filter(FilterMode::Nearest);
    message.set_filter(FilterMode::Nearest);
    let sound_effects = SoundEffects::new().await;
    let player = &mut Player::new().await;
    let world = &mut World::new().await;
    let mut game_state = GameState::new(music_player, sound_effects);
    game_state.music_player_mut().play();

    loop {
        clear_background(BLACK);

        // Update game state
        game_state.update(player, world);

        // Draw current scene
        game_state.draw(&background, &message, player, world);

        next_frame().await
    }
}

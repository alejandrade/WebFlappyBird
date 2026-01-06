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

const FIXED_DELTA: f32 = 1.0 / 60.0; // 60 FPS fixed timestep
const MAX_FRAME_TIME: f32 = 0.25; // Cap at 250ms to prevent spiral of death
const TARGET_FPS: f32 = 60.0; // Target FPS for rendering
const TARGET_FRAME_TIME: f32 = 1.0 / TARGET_FPS; // Time per frame at target FPS

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
    
    let mut accumulator = 0.0;
    let mut frame_count = 0;
    let mut fps_timer = 0.0;

    loop {
        let frame_start = std::time::Instant::now();
        let delta = get_frame_time();
        
        // Update music player every frame (not fixed timestep)
        game_state.update_music();
        
        // Check inputs every frame to catch them even if they're missed during fixed updates
        game_state.check_inputs_every_frame();
        
        accumulator += delta;
        
        if accumulator > MAX_FRAME_TIME {
            accumulator = MAX_FRAME_TIME;
            println!("[WARNING] Frame time exceeded MAX_FRAME_TIME, capped at {:.3}s", MAX_FRAME_TIME);
        }
        
        let mut fixed_updates_this_frame = 0;
        while accumulator >= FIXED_DELTA {
            game_state.update(FIXED_DELTA);
            accumulator -= FIXED_DELTA;
            fixed_updates_this_frame += 1;
        }
        
        if fixed_updates_this_frame > 1 {
            println!(
                "[FIXED-UPDATE] Multiple fixed updates this frame: {} (delta: {:.4}s, accumulator: {:.4}s)",
                fixed_updates_this_frame, delta, accumulator
            );
        }
        
        let alpha = accumulator / FIXED_DELTA;
        
        clear_background(BLACK);
        game_state.draw(&message, alpha);
        next_frame().await;
        
        // Cap FPS by sleeping if frame completed too quickly
        let frame_duration = frame_start.elapsed();
        if frame_duration.as_secs_f32() < TARGET_FRAME_TIME {
            let sleep_duration = std::time::Duration::from_secs_f32(TARGET_FRAME_TIME - frame_duration.as_secs_f32());
            std::thread::sleep(sleep_duration);
        }
        
        frame_count += 1;
        fps_timer += delta;
        if fps_timer >= 1.0 {
            let fps = frame_count as f32 / fps_timer;
            println!("FPS: {:.2}", fps);
            frame_count = 0;
            fps_timer = 0.0;
        }
    }
}

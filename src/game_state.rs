use crate::assets::Assets;
use crate::background_texture_atlas::BackgroundTextureAtlas;
use crate::components::Node;
use crate::music_player::MusicPlayer;
use crate::player::Player;
use crate::sound_effects::SoundEffects;
use crate::world::World;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use macroquad::prelude::*;

pub enum GameState {
    StartScreen(StartScreenState),
    Playing(PlayingState),
    GameOver(GameOverState),
}

pub struct StartScreenState {
    music_player: MusicPlayer,
    sound_effects: SoundEffects,
    world: World,
    player: Player,
}
pub struct PlayingState {
    music_player: MusicPlayer,
    sound_effects: SoundEffects,
    world: World,
    player: Player,
}
pub struct GameOverState {
    music_player: MusicPlayer,
    sound_effects: SoundEffects,
    world: World,
    player: Player,
}

impl GameState {
    pub async fn new() -> Self {
        let background_texture_atlas = BackgroundTextureAtlas::new().await;
        let player = Player::new().await;
        let world = World::new(background_texture_atlas).await;
        let music_player = MusicPlayer::new("assets/music", 2.0)
            .await
            .expect("Failed to load music");
        let sound_effects = SoundEffects::new().await;
        GameState::StartScreen(StartScreenState {
            music_player,
            sound_effects,
            player,
            world,
        })
    }

    pub fn update(self) -> Self {
        match self {
            GameState::StartScreen(mut state) => {
                state.music_player.play();
                state.music_player.update();
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    return GameState::Playing(PlayingState {
                        music_player: state.music_player,
                        sound_effects: state.sound_effects,
                        player: state.player,
                        world: state.world,
                    });
                }
                let dt = get_frame_time();
                let player = &mut state.player;
                let world = &mut state.world;

                player.restart();
                world.restart();
                return GameState::StartScreen(state);
            }
            GameState::Playing(mut state) => {
                state.music_player.play();
                state.music_player.update();
                let dt = get_frame_time();
                let player = &mut state.player;
                let world = &mut state.world;

                player.update(dt);
                world.update(dt);

                // Play sound when score increases
                if world.player_passed_pipes(player) {
                    state.sound_effects.play_point();
                }

                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    state.sound_effects.play_wing();
                }

                if world.touched(player) {
                    state.sound_effects.play_hit();
                    player.dead();
                    world.end();
                    state.music_player.stop();
                    state.sound_effects.play_death();
                    return GameState::GameOver(GameOverState {
                        music_player: state.music_player,
                        sound_effects: state.sound_effects,
                        world: state.world,
                        player: state.player,
                    });
                }
                return GameState::Playing(state);
            }
            GameState::GameOver(mut state) => {
                state.music_player.play();
                state.music_player.update();
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    state.music_player.next();
                    return GameState::StartScreen(StartScreenState {
                        music_player: state.music_player,
                        sound_effects: state.sound_effects,
                        player: state.player,
                        world: state.world,
                    });
                }
                let dt = get_frame_time();
                let player = &mut state.player;
                let world = &mut state.world;
                player.update(dt);
                return GameState::GameOver(state);
            }
        }
    }

    pub fn draw(&mut self, message: &Texture2D, assets: &Assets) {
        match self {
            GameState::StartScreen(state) => {
                state.world.draw();
                let msg_x = (SCREEN_WIDTH - message.width()) / 2.0;
                let msg_y = (SCREEN_HEIGHT - message.height()) / 2.0 - 50.0;
                draw_texture(message, msg_x, msg_y, WHITE);
            }
            GameState::Playing(state) => {
                state.world.draw();
                state.player.draw();
            }
            GameState::GameOver(state) => {
                // Draw game over screen
                state.world.draw();
                state.player.draw();

                // Draw gameover image centered
                let gameover_x = (SCREEN_WIDTH - assets.gameover_texture.width()) / 2.0;
                let gameover_y = (SCREEN_HEIGHT - assets.gameover_texture.height()) / 2.0 - 100.0;
                draw_texture(&assets.gameover_texture, gameover_x, gameover_y, WHITE);

                // Draw instructions (two lines)
                let font_size = 20.0;
                let line_spacing = 25.0;

                let line1 = "Press SPACE or click";
                let line2 = "to continue";

                let line1_dimensions = measure_text(line1, None, font_size as u16, 1.0);
                let line2_dimensions = measure_text(line2, None, font_size as u16, 1.0);

                let line1_x = (SCREEN_WIDTH - line1_dimensions.width) / 2.0;
                let line2_x = (SCREEN_WIDTH - line2_dimensions.width) / 2.0;
                let start_y = gameover_y + assets.gameover_texture.height() + 50.0;

                draw_text(line1, line1_x, start_y, font_size, WHITE);
                draw_text(line2, line2_x, start_y + line_spacing, font_size, WHITE);
            }
        }
    }
}

use macroquad::prelude::*;
use crate::components::Node;
use crate::music_player::MusicPlayer;
use crate::player::Player;
use crate::sound_effects::SoundEffects;
use crate::world::World;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameScene {
    StartScreen,
    Playing,
    GameOver,
}

pub struct GameState {
    scene: GameScene,
    music_player: MusicPlayer,
    sound_effects: SoundEffects,
}

impl GameState {
    pub fn new(music_player: MusicPlayer,
                     sound_effects: SoundEffects) -> Self {
        Self {
            scene: GameScene::StartScreen,
            music_player,
            sound_effects
        }
    }

    fn handle_input(&mut self) {
        match self.scene {
            GameScene::StartScreen => {
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    self.scene = GameScene::Playing;
                }
            }
            GameScene::Playing => {

            }
            GameScene::GameOver => {
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    self.scene = GameScene::StartScreen;
                    self.music_player.next();
                }
            }
        }
    }

    pub fn update(&mut self, player: &mut Player, world: &mut World) {
        self.music_player.update();
        self.handle_input();
        let dt = get_frame_time();

        match self.scene {
            GameScene::StartScreen => {
                player.restart();
                world.restart();
            }
            GameScene::Playing => {

                player.update(dt);
                world.update(dt);

                // Play sound when score increases
                if world.player_passed_pipes(player) {
                    self.sound_effects.play_point();
                }

                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    self.sound_effects.play_wing();
                }

                if world.touched(player) {
                    self.sound_effects.play_hit();
                    player.dead();
                    world.end();
                    self.music_player.stop();
                    self.scene = GameScene::GameOver;
                    self.sound_effects.play_death();
                }
            }
            GameScene::GameOver => {
                // Continue updating player so they fall with gravity
                player.update(dt);
            }
        }
    }

    pub fn draw(&self, background: &Texture2D, message: &Texture2D,
                player: &mut Player, world: &mut World) {
        // Draw background (always visible)
        draw_texture_ex(
            background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        match self.scene {
            GameScene::StartScreen => {
                // Draw message centered
                let msg_x = (screen_width() - message.width()) / 2.0;
                let msg_y = (screen_height() - message.height()) / 2.0 - 50.0;
                draw_texture(message, msg_x, msg_y, WHITE);
            }
            GameScene::Playing => {
                player.draw();
                world.draw();
            }
            GameScene::GameOver => {
                // Draw game over screen
                player.draw();
                world.draw();
            }
        }
    }

    pub fn music_player_mut(&mut self) -> &mut MusicPlayer {
        &mut self.music_player
    }
}
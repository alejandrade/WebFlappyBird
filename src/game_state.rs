use macroquad::prelude::*;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::background_texture_atlas::BackgroundTextureAtlas;
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
    world: World,
    player: Player,
    gameover_texture: Texture2D,
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
        let gameover_texture = load_texture("assets/sprites/gameover.png")
            .await
            .expect("Failed to load gameover texture");
        Self {
            scene: GameScene::StartScreen,
            music_player,
            sound_effects,
            player,
            world,
            gameover_texture
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

    pub fn update(&mut self) {
        self.music_player.play();
        self.music_player.update();
        self.handle_input();
        let dt = get_frame_time();
        let player  = &mut self.player;
        let world = &mut self.world;

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
                
                player.update(dt);
            }
        }
    }

    pub fn draw(&mut self, message: &Texture2D) {
        match self.scene {
            GameScene::StartScreen => {
                self.world.draw();
                let msg_x = (SCREEN_WIDTH - message.width()) / 2.0;
                let msg_y = (SCREEN_HEIGHT - message.height()) / 2.0 - 50.0;
                draw_texture(message, msg_x, msg_y, WHITE);
            }
            GameScene::Playing => {
                self.world.draw();
                self.player.draw();

            }
            GameScene::GameOver => {
                // Draw game over screen
                self.world.draw();
                self.player.draw();

                // Draw gameover image centered
                let gameover_x = (SCREEN_WIDTH - self.gameover_texture.width()) / 2.0;
                let gameover_y = (SCREEN_HEIGHT - self.gameover_texture.height()) / 2.0 - 100.0;
                draw_texture(&self.gameover_texture, gameover_x, gameover_y, WHITE);

                // Draw instructions (two lines)
                let font_size = 20.0;
                let line_spacing = 25.0;

                let line1 = "Press SPACE or click";
                let line2 = "to continue";

                let line1_dimensions = measure_text(line1, None, font_size as u16, 1.0);
                let line2_dimensions = measure_text(line2, None, font_size as u16, 1.0);

                let line1_x = (SCREEN_WIDTH - line1_dimensions.width) / 2.0;
                let line2_x = (SCREEN_WIDTH - line2_dimensions.width) / 2.0;
                let start_y = gameover_y + self.gameover_texture.height() + 50.0;

                draw_text(line1, line1_x, start_y, font_size, WHITE);
                draw_text(line2, line2_x, start_y + line_spacing, font_size, WHITE);
            }
        }
    }

}
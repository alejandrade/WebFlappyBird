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
    // Input buffering: Store inputs detected between fixed updates
    input_buffer_space: bool,
    input_buffer_mouse: bool,
    // Debug: Track input detection
    debug_input_frame_count: u64,
    debug_fixed_update_count: u64,
    debug_inputs_detected: Vec<String>,
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
            gameover_texture,
            input_buffer_space: false,
            input_buffer_mouse: false,
            debug_input_frame_count: 0,
            debug_fixed_update_count: 0,
            debug_inputs_detected: Vec::new(),
        }
    }

    pub fn update_music(&mut self) {
        self.music_player.play();
        self.music_player.update();
    }

    /// Check inputs every frame (not just during fixed updates)
    /// Buffer inputs so they can be processed during fixed updates
    pub fn check_inputs_every_frame(&mut self) {
        self.debug_input_frame_count += 1;
        
        let space_pressed = is_key_pressed(KeyCode::Space);
        let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        
        // Buffer inputs - set to true if pressed, don't clear until processed
        if space_pressed {
            self.input_buffer_space = true;
        }
        if mouse_pressed {
            self.input_buffer_mouse = true;
        }
        
        if space_pressed || mouse_pressed {
            let input_type = if space_pressed && mouse_pressed {
                "SPACE+MOUSE"
            } else if space_pressed {
                "SPACE"
            } else {
                "MOUSE"
            };
            
            let msg = format!(
                "[INPUT-DETECTED] Frame {}: {} pressed (Scene: {:?}) - BUFFERED",
                self.debug_input_frame_count, input_type, self.scene
            );
            println!("{}", msg);
            self.debug_inputs_detected.push(msg);
            
            // Keep only last 10 inputs for debugging
            if self.debug_inputs_detected.len() > 10 {
                self.debug_inputs_detected.remove(0);
            }
        }
    }

    fn handle_input(&mut self) -> (bool, bool) {
        self.debug_fixed_update_count += 1;
        
        // Read buffered inputs (don't clear yet - will be cleared at end of update)
        // This ensures inputs detected between fixed updates are still processed
        let space_pressed = self.input_buffer_space;
        let mouse_pressed = self.input_buffer_mouse;
        
        if space_pressed || mouse_pressed {
            let input_type = if space_pressed && mouse_pressed {
                "SPACE+MOUSE"
            } else if space_pressed {
                "SPACE"
            } else {
                "MOUSE"
            };
            
            println!(
                "[INPUT-PROCESSED] Fixed update {}: {} processed from buffer (Scene: {:?}, Frame: {})",
                self.debug_fixed_update_count, input_type, self.scene, self.debug_input_frame_count
            );
        }
        
        match self.scene {
            GameScene::StartScreen => {
                if space_pressed || mouse_pressed {
                    println!("[SCENE-CHANGE] StartScreen -> Playing");
                    self.scene = GameScene::Playing;
                }
            }
            GameScene::Playing => {
            }
            GameScene::GameOver => {
                if space_pressed || mouse_pressed {
                    println!("[SCENE-CHANGE] GameOver -> StartScreen");
                    self.scene = GameScene::StartScreen;
                    self.music_player.next();
                }
            }
        }
        
        (space_pressed, mouse_pressed)
    }

    pub fn update(&mut self, dt: f32) {
        // Get buffered input state (used for scene changes and jump)
        let (space_pressed, mouse_pressed) = self.handle_input();
        let player  = &mut self.player;
        let world = &mut self.world;

        match self.scene {
            GameScene::StartScreen => {
                player.restart();
                world.restart();
            }
            GameScene::Playing => {
                // Handle jump input from buffer (before physics update)
                if space_pressed || mouse_pressed {
                    player.handle_jump();
                    println!(
                        "[WING-SOUND] Fixed update {}: Wing sound triggered (Frame: {})",
                        self.debug_fixed_update_count, self.debug_input_frame_count
                    );
                    self.sound_effects.play_wing();
                }

                player.update(dt);
                world.update(dt);

                // Play sound when score increases
                if world.player_passed_pipes(player) {
                    self.sound_effects.play_point();
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
        
        // Clear input buffer after all uses (one-shot inputs)
        self.input_buffer_space = false;
        self.input_buffer_mouse = false;
    }

    pub fn draw(&mut self, message: &Texture2D, _alpha: f32) {
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
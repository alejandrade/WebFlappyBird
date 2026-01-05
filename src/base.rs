use crate::SCREEN_HEIGHT;
use crate::base_texture_atlas::BaseTextureAtlas;
use crate::components::Node;
use crate::player::Player;
use macroquad::color::WHITE;
use macroquad::math::vec2;
use macroquad::prelude::{DrawTextureParams, draw_texture_ex};

pub struct Base {
    pub base_texture_atlas: BaseTextureAtlas,
    pub velocity: u16,
    pub height: f32,
    pub x1: f32,
    pub x2: f32,
    pub stopped: bool,
}

impl Base {
    pub async fn new(base_texture_atlas: BaseTextureAtlas, velocity: u16) -> Base {
        let height = base_texture_atlas.height;
        let width = base_texture_atlas.width;
        Base {
            base_texture_atlas,
            velocity,
            height,
            x1: 0.0,
            x2: width,
            stopped: false,
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn restart(&mut self) {
        self.stopped = false;
    }

    pub fn touched(&self, player: &Player) -> bool {
        let width = self.base_texture_atlas.width;
        let height = self.base_texture_atlas.height;
        let y = SCREEN_HEIGHT - height;

        let player_x = player.position.x;
        let player_y = player.position.y;
        let player_width = player.bird_renderer.width;
        let player_height = player.bird_renderer.height;

        // Check collision with first base
        let collision1 = self.x1 < player_x + player_width
            && self.x1 + width > player_x
            && y < player_y + player_height
            && y + height > player_y;

        // Check collision with second base
        let collision2 = self.x2 < player_x + player_width
            && self.x2 + width > player_x
            && y < player_y + player_height
            && y + height > player_y;

        collision1 || collision2
    }
}

impl Node for Base {
    fn update(&mut self, dt: f32) {
        if !self.stopped {
            let speed = self.velocity as f32 * dt;
            self.x1 -= speed;
            self.x2 -= speed;

            let width = self.base_texture_atlas.width;

            if self.x1 + width <= 0.0 {
                self.x1 = self.x2 + width;
            }

            if self.x2 + width <= 0.0 {
                self.x2 = self.x1 + width;
            }
        }
    }

    fn draw(&mut self) {
        let width = self.base_texture_atlas.width;
        let height = self.base_texture_atlas.height;
        let y = SCREEN_HEIGHT - height;
        let texture = &self.base_texture_atlas.texture;

        // Draw first base
        draw_texture_ex(
            texture,
            self.x1,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                ..Default::default()
            },
        );

        // Draw second base
        draw_texture_ex(
            texture,
            self.x2,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                ..Default::default()
            },
        );
    }
}

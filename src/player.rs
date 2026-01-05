use crate::assets::Assets;
use crate::components::Node;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use macroquad::color::WHITE;
use macroquad::input::{KeyCode, MouseButton, is_key_pressed, is_mouse_button_pressed};
use macroquad::math::{Vec2, vec2};
use macroquad::prelude::draw_texture_ex;
use macroquad::texture::DrawTextureParams;

pub struct Player {
    pub alive: bool,
    pub position: Vec2,
    pub jump_force: f32,
    pub vel: Vec2,
    pub start_position: Vec2,
    pub rotation: f32,
    pub width: f32,
    pub height: f32,
}

impl Player {
    pub(crate) fn new(assets: &Assets) -> Self {
        let x = SCREEN_WIDTH / 2.0 - assets.bird_renderer.width / 2.0;
        let y = SCREEN_HEIGHT / 2.0 - assets.bird_renderer.height / 2.0;

        Self {
            alive: true,
            position: Vec2::new(x, y),
            vel: Vec2::new(0.0, 0.0),
            jump_force: -300.0,
            start_position: Vec2::new(x, y),
            rotation: 0.0,
            width: assets.bird_renderer.width,
            height: assets.bird_renderer.height,
        }
    }

    pub fn dead(&mut self) {
        if self.alive {
            self.vel.x = -50.0;
            self.vel.y = self.jump_force;
        }
        self.alive = false;
    }
}
impl Node for Player {
    fn update(&mut self, dt: f32) {
        let gravity = 800.0;
        let max_fall_speed = 1500.0;
        let max_horizontal_speed = 300.0;

        self.vel.y += gravity * dt;

        // Cap velocities to prevent overflow
        self.vel.y = self.vel.y.clamp(-max_fall_speed, max_fall_speed);
        self.vel.x = self
            .vel
            .x
            .clamp(-max_horizontal_speed, max_horizontal_speed);

        self.position.y += self.vel.y * dt;
        self.position.x += self.vel.x * dt;

        if self.alive {
            // Only allow jumping when alive
            if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                self.vel.y = if self.vel.y < 0.0 {
                    self.jump_force - 100.0
                } else {
                    self.jump_force
                };

                println!("{}", self.vel.y)
            }
        } else {
            self.rotation -= 1.5 * dt;
        }
    }

    fn draw(&mut self, assets: &Assets) {
        let (texture, rotation) = if self.vel.y < -50.0 {
            (&assets.bird_renderer.bird_sprites.downflap_texture, 25.0)
        } else if self.vel.y > 50.0 {
            (&assets.bird_renderer.bird_sprites.upflap_texture, -25.0)
        } else {
            (&assets.bird_renderer.bird_sprites.midflap_texture, 0.0)
        };
        self.rotation = rotation;

        let width = texture.width();
        let height = texture.height();

        draw_texture_ex(
            texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                rotation: self.rotation,
                ..Default::default()
            },
        );
    }
}

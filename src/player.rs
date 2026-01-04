use macroquad::color::WHITE;
use macroquad::input::{is_key_pressed, is_mouse_button_pressed, KeyCode, MouseButton};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::draw_texture_ex;
use macroquad::texture::DrawTextureParams;
use macroquad::window::{screen_height, screen_width};
use crate::bird_texture_atlas::BirdTextureAtlas;
use crate::components::Node;

pub struct Player {
    pub alive: bool,
    pub position: Vec2,
    pub jump_force: f32,
    pub vel: Vec2,
    pub bird_renderer: BirdTextureAtlas,
    pub start_position: Vec2,
    pub rotation: f32,
}

impl Player {
    pub(crate) async fn new() -> Self {
        let bird_renderer = BirdTextureAtlas::new().await;
        let x = screen_width() / 2.0 - bird_renderer.width / 2.0;
        let y = screen_height() / 2.0 - bird_renderer.height / 2.0;


        Self {
            alive: true,
            position: Vec2::new(x, y),
            vel: Vec2::new(0.0, 0.0),
            jump_force: -250.0,
            bird_renderer,
            start_position: Vec2::new(x, y),
            rotation: 0.0,
        }
    }

    pub fn dead(&mut self)  {
        if self.alive {
            self.vel.x = -50.0;
            self.vel.y = self.jump_force;
        }
        self.alive = false;
    }

    pub fn restart(&mut self) {
        self.alive = true;
        self.position = self.start_position;
        self.vel = Vec2::new(0.0, 0.0);
        self.rotation = 0.0;
    }
}
impl Node for Player {

    fn update(&mut self, dt: f32) {
        let gravity = 500.0;
        let max_fall_speed = 1000.0;
        let max_horizontal_speed = 300.0;

        self.vel.y += gravity * dt;

        // Cap velocities to prevent overflow
        self.vel.y = self.vel.y.clamp(-max_fall_speed, max_fall_speed);
        self.vel.x = self.vel.x.clamp(-max_horizontal_speed, max_horizontal_speed);

        self.position.y += self.vel.y * dt;
        self.position.x += self.vel.x * dt;

        if self.alive {
            // Only allow jumping when alive
            if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                self.vel.y = self.jump_force
            }
        } else {
            self.rotation -= 1.5 * dt;
        }
    }

    fn draw(&mut self) {
        let texture = if self.vel.y < -50.0 {
            &self.bird_renderer.bird_sprites.downflap_texture
        } else if self.vel.y > 50.0 {
            &self.bird_renderer.bird_sprites.upflap_texture
        } else {
            &self.bird_renderer.bird_sprites.midflap_texture
        };

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
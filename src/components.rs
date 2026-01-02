use macroquad::math::Vec2;
use macroquad::prelude::{screen_height, screen_width};
use crate::bird_texture_atlas::BirdTextureAtlas;

pub trait Node {
    fn update(&mut self, dt: f32);
    fn draw(&mut self);
}
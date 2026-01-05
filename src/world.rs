use crate::assets::Assets;
use crate::background_texture_atlas::BackgroundType;
use crate::base::Base;
use crate::components::Node;
use crate::pipes::{Pipe, PipeLocation};
use crate::player::Player;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use macroquad::color::WHITE;
use macroquad::prelude::vec2;
use macroquad::rand::gen_range;
use macroquad::texture::{DrawTextureParams, draw_texture_ex};
use std::rc::Rc;

pub struct World {
    pub score: u32,
    pub timer: f32,
    pub pipes: Vec<(Pipe, Pipe)>,
    pub pipe_spawn_time: f32,
    pub velocity: u16,
    pub base: Base,
    pub last_pipe_location_index: usize,
}

pub const VELOCITY: u16 = 130;

impl World {
    pub fn new(assets: &Assets) -> Self {
        let base = Base::new(&assets.base_texture_atlas, VELOCITY);
        World {
            score: 0,
            timer: 0.0,
            pipes: Vec::new(),
            pipe_spawn_time: 2.0,
            velocity: VELOCITY,
            base,
            last_pipe_location_index: 4, // Start at Mid
        }
    }

    pub fn touched(&self, player: &Player) -> bool {
        // Check if player flew off screen (top or bottom)
        let off_screen = player.position.y < 0.0 || player.position.y > SCREEN_HEIGHT;

        self.pipes
            .iter()
            .any(|(pipe1, pipe2)| pipe1.touched(player) || pipe2.touched(player))
            || self.base.touched(player)
            || off_screen
    }

    pub fn end(&mut self) {
        self.pipes.iter_mut().for_each(|(pipe1, pipe2)| {
            pipe1.stop();
            pipe2.stop();
        });

        self.base.stop();
    }

    pub fn restart(&mut self) {
        self.pipes.retain(|(_pipe1, _pipe2)| false);
        self.pipes.iter_mut().for_each(|(pipe1, pipe2)| {
            pipe2.restart();
            pipe1.restart();
        });
        self.base.restart();
        self.score = 0;
    }

    pub fn player_passed_pipes(&mut self, player: &Player) -> bool {
        let mut passed = false;
        self.pipes.iter_mut().for_each(|(pipe1, _pipe2)| {
            if pipe1.has_passed(player) && !pipe1.passed {
                pipe1.passed = true;
                passed = true;
                self.score += 1;
            }
        });
        passed
    }

    fn draw_score(&self, assets: &Assets) {
        use macroquad::prelude::*;

        // Convert score to digits
        let score_str = self.score.to_string();
        let digits: Vec<u32> = score_str.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let digit_width = assets.number_texture_atlas.width;
        let total_width = digits.len() as f32 * digit_width;
        let screen_w = SCREEN_WIDTH;
        let start_x = (screen_w - total_width) / 2.0;
        let y = 50.0;

        for (i, &digit) in digits.iter().enumerate() {
            let texture = &assets.number_texture_atlas.number_sprites.digits[digit as usize];
            let x = start_x + (i as f32 * digit_width);

            draw_texture_ex(
                texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(digit_width, assets.number_texture_atlas.height)),
                    ..Default::default()
                },
            );
        }
    }
}

impl Node for World {
    fn update(&mut self, dt: f32) {
        self.timer += dt;

        if self.timer >= self.pipe_spawn_time {
            // Generate next location within ±4 positions of last
            // Removed concept of range limit - always select from all possible locations (0-9)
            let r = gen_range(0, 10); // Generates a number from 0 to 9 inclusive
            let location = match r {
                0 => PipeLocation::VeryHigh,
                1 => PipeLocation::High,
                2 => PipeLocation::HighMid,
                3 => PipeLocation::UpperMid,
                4 => PipeLocation::Mid,
                5 => PipeLocation::LowerMid,
                6 => PipeLocation::LowMid,
                7 => PipeLocation::Low,
                8 => PipeLocation::VeryLow,
                _ => PipeLocation::Lowest,
            };
            println!("Generated PipeLocation: {:?}", location);

            self.last_pipe_location_index = r;
            let base_height = self.base.height;
            let score = self.score;
            self.pipes.push((
                Pipe::new(
                    Rc::clone(&self.pipe_texture_atlas),
                    false,
                    self.velocity,
                    location.clone(),
                    base_height,
                    score,
                ),
                Pipe::new(
                    Rc::clone(&self.pipe_texture_atlas),
                    true,
                    self.velocity,
                    location,
                    base_height,
                    score,
                ),
            ));
            self.timer = 0.0;
        }

        self.pipes.iter_mut().for_each(|(pipe1, pipe2)| {
            pipe1.update(dt);
            pipe2.update(dt);
        });
        self.base.update(dt);

        self.pipes.retain(|(pipe1, _)| !pipe1.is_off_screen());
    }

    fn draw(&mut self, assets: &Assets) {
        let background_texture = if (self.score / 10) % 2 != 0 {
            assets.background_texture_atlas
                .get_texture_2d(BackgroundType::Night)
        } else {
            assets.background_texture_atlas
                .get_texture_2d(BackgroundType::Day)
        };

        draw_texture_ex(
            background_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(SCREEN_WIDTH, SCREEN_HEIGHT)),
                ..Default::default()
            },
        );

        self.pipes.iter_mut().for_each(|(pipe1, pipe2)| {
            pipe1.draw(&assets);
            pipe2.draw(&assets);
        });

        self.base.draw(&assets);

        // Draw score if greater than 0
        if self.score > 0 {
            self.draw_score(&assets);
        }
    }
}

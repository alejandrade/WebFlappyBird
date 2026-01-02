use std::rc::Rc;
use macroquad::color::WHITE;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{draw_texture_ex, DrawTextureParams};
use macroquad::window::{screen_height, screen_width};
use crate::components::Node;
use crate::pipe_texture_atlas::PipeTextureAtlas;
use crate::player::Player;

#[derive(Clone)]
pub enum PipeLocation {
    VeryHigh,
    High,
    HighMid,
    UpperMid,
    Mid,
    LowerMid,
    LowMid,
    Low,
    VeryLow,
    Lowest,
}


pub struct Pipe {
    pub pipe_texture_atlas: Rc<PipeTextureAtlas>,
    pub velocity: u16,
    pub reflected: bool,
    position: Vec2,
    pipe_location: PipeLocation,
    base_height: f32,
    stopped: bool,
    pub passed: bool,
}


impl Pipe {
    pub fn new(pipe_texture_atlas: Rc<PipeTextureAtlas>,
               reflected: bool,
               velocity: u16,
               pipe_location: PipeLocation,
               base_height: f32) -> Pipe {

        // Spawn off-screen to the right
        let x = screen_width();
        let y = 0.0; // Will be calculated in draw based on location and reflection

        Pipe {
            pipe_texture_atlas,
            velocity,
            reflected,
            position: vec2(x, y),
            pipe_location,
            base_height,
            stopped: false,
            passed: false,
        }
    }

    pub fn has_passed(&mut self, player: &Player) -> bool {
        self.position.x < player.position.x
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn restart(&mut self) {
        self.stopped = false;
    }

    pub fn is_off_screen(&self) -> bool {
        self.position.x + self.pipe_texture_atlas.width < 0.0
    }

    fn get_pipe_y_and_height(&self) -> (f32, f32) {
        let screen_h = screen_height();
        let playable_height = screen_h - self.base_height;
        let gap_size = 100.0;

        // Set bounds to ensure gap stays in visible area with room for pipes
        // Min: gap should be at least gap_size from top (room for top pipe to show)
        // Max: gap should be at least gap_size from base (room for bottom pipe to show)
        let min_gap_y = gap_size * 1.5;
        let max_gap_y = playable_height - (gap_size * 1.5);
        let gap_range = max_gap_y - min_gap_y;

        // Percentage within the valid bounds
        let percentage = match self.pipe_location {
            PipeLocation::VeryHigh => 0.0,
            PipeLocation::High => 0.111,
            PipeLocation::HighMid => 0.222,
            PipeLocation::UpperMid => 0.333,
            PipeLocation::Mid => 0.444,
            PipeLocation::LowerMid => 0.555,
            PipeLocation::LowMid => 0.666,
            PipeLocation::Low => 0.777,
            PipeLocation::VeryLow => 0.888,
            PipeLocation::Lowest => 1.0,
        };

        let gap_center_y = min_gap_y + (gap_range * percentage);

        let pipe_height = self.pipe_texture_atlas.height;

        if self.reflected {
            // Top pipe (upside down) - bottom edge at gap top
            let gap_top = gap_center_y - (gap_size / 2.0);
            let y = gap_top - pipe_height;
            (y, pipe_height)
        } else {
            // Bottom pipe (normal) - top edge at gap bottom
            let y = gap_center_y + (gap_size / 2.0);
            (y, pipe_height)
        }
    }

    pub fn touched(&self, player: &Player) -> bool {
        let (pipe_y, pipe_height) = self.get_pipe_y_and_height();
        let pipe_x = self.position.x;
        let pipe_width = self.pipe_texture_atlas.width;

        let player_x = player.position.x;
        let player_y = player.position.y;
        let player_width = player.bird_renderer.width;
        let player_height = player.bird_renderer.height;

        // AABB collision detection
        pipe_x < player_x + player_width
            && pipe_x + pipe_width > player_x
            && pipe_y < player_y + player_height
            && pipe_y + pipe_height > player_y
    }
}

impl Node for Pipe {
    fn update(&mut self, dt: f32) {
        // Only move if not stopped
        if !self.stopped {
            self.position.x -= self.velocity as f32 * dt;
        }
    }

    fn draw(&mut self) {
        let screen_h = screen_height();
        let playable_height = screen_h - self.base_height;

        let gap_size = 100.0;

        // Set bounds to ensure gap stays in visible area with room for pipes
        // Min: gap should be at least gap_size from top (room for top pipe to show)
        // Max: gap should be at least gap_size from base (room for bottom pipe to show)
        let min_gap_y = gap_size * 1.5;
        let max_gap_y = playable_height - (gap_size * 1.5);
        let gap_range = max_gap_y - min_gap_y;

        // Percentage within the valid bounds
        let percentage = match self.pipe_location {
            PipeLocation::VeryHigh => 0.0,
            PipeLocation::High => 0.111,
            PipeLocation::HighMid => 0.222,
            PipeLocation::UpperMid => 0.333,
            PipeLocation::Mid => 0.444,
            PipeLocation::LowerMid => 0.555,
            PipeLocation::LowMid => 0.666,
            PipeLocation::Low => 0.777,
            PipeLocation::VeryLow => 0.888,
            PipeLocation::Lowest => 1.0,
        };

        let gap_center_y = min_gap_y + (gap_range * percentage);

        let pipe_width = self.pipe_texture_atlas.width;
        let pipe_height = self.pipe_texture_atlas.height;
        let texture = &self.pipe_texture_atlas.pipe_sprites.green_texture;

        if self.reflected {
            // Top pipe (upside down) - bottom edge at gap top
            let gap_top = gap_center_y - (gap_size / 2.0);
            let y = gap_top - pipe_height;

            draw_texture_ex(
                texture,
                self.position.x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(pipe_width, pipe_height)),
                    flip_y: true,
                    ..Default::default()
                },
            );
        } else {
            // Bottom pipe (normal) - top edge at gap bottom
            let y = gap_center_y + (gap_size / 2.0);

            draw_texture_ex(
                texture,
                self.position.x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(pipe_width, pipe_height)),
                    ..Default::default()
                },
            );
        }
    }
}
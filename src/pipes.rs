use crate::assets::Assets;
use crate::components::Node;
use crate::pipe_texture_atlas::PipeColor;
use crate::player::Player;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use macroquad::color::WHITE;
use macroquad::math::{Vec2, vec2};
use macroquad::prelude::{DrawTextureParams, draw_texture_ex};

#[derive(Clone, Debug)]
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
    pub velocity: u16,
    pub reflected: bool,
    position: Vec2,
    pipe_location: PipeLocation,
    base_height: f32,
    stopped: bool,
    score: u32,
    pub passed: bool,
    height: f32,
    width: f32,
}

const GAP_SIZE: f32 = 140.0;

impl Pipe {
    pub fn new(
        reflected: bool,
        velocity: u16,
        pipe_location: PipeLocation,
        base_height: f32,
        score: u32,
        assets: &Assets,
    ) -> Pipe {
        // Spawn off-screen to the right
        let x = SCREEN_WIDTH;
        let y = 0.0;

        Pipe {
            velocity,
            reflected,
            position: vec2(x, y),
            pipe_location,
            base_height,
            stopped: false,
            score,
            passed: false,
            height: assets.pipe_texture_atlas.height,
            width: assets.pipe_texture_atlas.width,
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

    pub fn is_off_screen(&self, assets: &Assets) -> bool {
        self.position.x + assets.pipe_texture_atlas.width < 0.0
    }

    fn get_pipe_y_and_height(&self) -> (f32, f32) {
        let screen_h = SCREEN_HEIGHT;
        let playable_height = screen_h - self.base_height;
        let gap_size = GAP_SIZE;

        // Valid range for the bottom of the top pipe
        let min_bottom_y = 20.0;
        let max_bottom_y = playable_height - gap_size - 20.0;
        let range = max_bottom_y - min_bottom_y;

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

        let top_pipe_bottom_y = min_bottom_y + (range * percentage);
        let pipe_height = self.height;

        if self.reflected {
            // Top pipe (upside down) - bottom edge at top_pipe_bottom_y
            let y = top_pipe_bottom_y - pipe_height;
            (y, pipe_height)
        } else {
            // Bottom pipe (normal) - top edge is gap_size below top pipe's bottom
            let y = top_pipe_bottom_y + gap_size;
            (y, pipe_height)
        }
    }

    pub fn touched(&self, player: &Player) -> bool {
        let (pipe_y, pipe_height) = self.get_pipe_y_and_height();
        let pipe_x = self.position.x;
        let pipe_width = self.width;

        let player_x = player.position.x;
        let player_y = player.position.y;
        let player_width = player.width;
        let player_height = player.height;

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

    fn draw(&mut self, assets: &Assets) {
        let (y, pipe_height) = self.get_pipe_y_and_height();
        let pipe_width = assets.pipe_texture_atlas.width;

        let texture = if (self.score / 5) % 2 != 0 {
            assets.pipe_texture_atlas.get_texture_2d(PipeColor::Green)
        } else {
            assets.pipe_texture_atlas.get_texture_2d(PipeColor::Red)
        };

        draw_texture_ex(
            &texture,
            self.position.x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(pipe_width, pipe_height)),
                flip_y: self.reflected,
                ..Default::default()
            },
        );
    }
}

use std::fs;
use macroquad::Error;
use macroquad::rand::gen_range;
use macroquad::texture::{load_texture, FilterMode, Texture2D};

pub enum BirdColor {
    YellowBird,
    BlueBird,
    RedBird,
}

impl BirdColor {
    // We return a &str because these are hardcoded constants
    pub fn as_str(&self) -> &'static str {
        match self {
            BirdColor::YellowBird => "yellowbird",
            BirdColor::BlueBird   => "bluebird",
            BirdColor::RedBird    => "redbird",
        }
    }

    pub fn get_random() -> Self {
        let r = gen_range(0, 3);
        match r {
            0 => Self::YellowBird,
            1 => Self::BlueBird,
            _ => Self::RedBird,
        }
    }
}

pub struct BirdSprites {
    pub downflap_texture: Texture2D,
    pub midflap_texture: Texture2D,
    pub upflap_texture: Texture2D
}


pub struct BirdTextureAtlas {
    pub bird_sprites: BirdSprites,
    pub width: f32,
    pub height: f32
}


impl BirdTextureAtlas {
    pub async fn new() -> BirdTextureAtlas {
        let random_bird_color = BirdColor::get_random();
        let random_bird_color_str = random_bird_color.as_str();
        let down = Self::get_texture(random_bird_color_str, "downflap").await;
        let mid = Self::get_texture(random_bird_color_str, "midflap").await;
        let up = Self::get_texture(random_bird_color_str, "upflap").await;

        let textures = [&down, &mid, &up];
        for t in textures {
            t.set_filter(FilterMode::Nearest);
        }

        let w = mid.width();
        let h = mid.height();

        BirdTextureAtlas {
            bird_sprites: BirdSprites {
                downflap_texture: down,
                midflap_texture: mid,
                upflap_texture: up
            },
            width: w,
            height: h
        }
    }

    pub async fn get_texture(bird_color: &str, text_type: &str) -> Texture2D {
        let path = format!("assets/sprites/{}-{}.png", bird_color, text_type);
        load_texture(&path)
            .await
            .expect("Failed to load texture")
    }
}
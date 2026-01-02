use macroquad::texture::{load_texture, FilterMode, Texture2D};

pub enum PipeColor {
    Green,
    Red,
}

impl PipeColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            PipeColor::Green => "green",
            PipeColor::Red => "red",
        }
    }
}

pub struct PipeSprites {
    pub green_texture: Texture2D,
    pub red_texture: Texture2D,
}

pub struct PipeTextureAtlas {
    pub pipe_sprites: PipeSprites,
    pub width: f32,
    pub height: f32,
}

impl PipeTextureAtlas {
    pub async fn new() -> PipeTextureAtlas {
        let green = Self::get_texture("green").await;
        let red = Self::get_texture("red").await;

        let textures = [&green, &red];
        for t in textures {
            t.set_filter(FilterMode::Nearest);
        }

        let w = green.width();
        let h = green.height();

        PipeTextureAtlas {
            pipe_sprites: PipeSprites {
                green_texture: green,
                red_texture: red,
            },
            width: w,
            height: h,
        }
    }

    pub async fn get_texture(pipe_color: &str) -> Texture2D {
        let path = format!("assets/sprites/pipe-{}.png", pipe_color);
        load_texture(&path)
            .await
            .expect("Failed to load texture")
    }
}
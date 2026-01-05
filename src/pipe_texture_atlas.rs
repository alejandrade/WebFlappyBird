use macroquad::texture::{FilterMode, Texture2D, load_texture};

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
        load_texture(&path).await.expect("Failed to load texture")
    }

    pub fn get_texture_2d(&self, pipe_color: PipeColor) -> &Texture2D {
        if pipe_color.as_str() == PipeColor::Green.as_str() {
            &self.pipe_sprites.green_texture
        } else {
            &self.pipe_sprites.red_texture
        }
    }
}

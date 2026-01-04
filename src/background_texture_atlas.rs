use macroquad::texture::{load_texture, FilterMode, Texture2D};

pub enum BackgroundType {
    Day,
    Night,
}

impl BackgroundType {
    pub fn as_str(&self) -> &'static str {
        match self {
            BackgroundType::Day => "day",
            BackgroundType::Night => "night",
        }
    }
}

pub struct BackgroundSprites {
    pub day_texture: Texture2D,
    pub night_texture: Texture2D,
}

pub struct BackgroundTextureAtlas {
    pub background_sprites: BackgroundSprites,
    pub width: f32,
    pub height: f32,
}

impl BackgroundTextureAtlas {
    pub async fn new() -> BackgroundTextureAtlas {
        let day = Self::get_texture("day").await;
        let night = Self::get_texture("night").await;

        let textures = [&day, &night];
        for t in textures {
            t.set_filter(FilterMode::Nearest);
        }

        let w = day.width();
        let h = day.height();

        BackgroundTextureAtlas {
            background_sprites: BackgroundSprites {
                day_texture: day,
                night_texture: night,
            },
            width: w,
            height: h,
        }
    }

    async fn get_texture(background_type: &str) -> Texture2D {
        let path = format!("assets/sprites/background-{}.png", background_type);
        load_texture(&path)
            .await
            .expect("Failed to load texture")
    }

    pub fn get_texture_2d(&self, background_type: BackgroundType) -> &Texture2D {
        if background_type.as_str() == BackgroundType::Day.as_str() {
            &self.background_sprites.day_texture
        } else {
            &self.background_sprites.night_texture
        }
    }
}
use macroquad::texture::{load_texture, FilterMode, Texture2D};

pub struct BaseTextureAtlas {
    pub texture: Texture2D,
    pub width: f32,
    pub height: f32,
}

impl BaseTextureAtlas {
    pub async fn new() -> BaseTextureAtlas {
        let texture = Self::get_texture().await;
        texture.set_filter(FilterMode::Nearest);

        let w = texture.width();
        let h = texture.height();

        BaseTextureAtlas {
            texture,
            width: w,
            height: h,
        }
    }

    pub async fn get_texture() -> Texture2D {
        let path = "assets/sprites/base.png";
        load_texture(path)
            .await
            .expect("Failed to load texture")
    }
}
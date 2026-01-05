use crate::bird_texture_atlas::BirdTextureAtlas;
use macroquad::prelude::*;

pub struct Assets {
    pub gameover_texture: Texture2D,
    pub bird_renderer: BirdTextureAtlas,
}

impl Assets {
    pub async fn load() -> Self {
        Assets {
            gameover_texture: load_texture("assets/sprites/gameover.png")
                .await
                .expect("Failed to load gameover texture"),
            bird_renderer: BirdTextureAtlas::new().await,
        }
    }
}

use macroquad::prelude::*;

pub struct Assets {
    pub gameover_texture: Texture2D,
}

impl Assets {
    pub async fn load() -> Self {
        Assets {
            gameover_texture: load_texture("assets/sprites/gameover.png")
                .await
                .expect("Failed to load gameover texture"),
        }
    }
}

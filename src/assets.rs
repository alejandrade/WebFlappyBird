use crate::background_texture_atlas::BackgroundTextureAtlas;
use crate::base_texture_atlas::BaseTextureAtlas;
use crate::bird_texture_atlas::BirdTextureAtlas;
use crate::number_texture_atlas::NumberTextureAtlas;
use crate::pipe_texture_atlas::PipeTextureAtlas;
use macroquad::prelude::*;
use std::rc::Rc;

pub struct Assets {
    pub gameover_texture: Texture2D,
    pub bird_renderer: BirdTextureAtlas,
    pub base_texture_atlas: BaseTextureAtlas,
    pub background_texture_atlas: BackgroundTextureAtlas,
    pub pipe_texture_atlas: Rc<PipeTextureAtlas>,
    pub number_texture_atlas: NumberTextureAtlas,
}

impl Assets {
    pub async fn load() -> Self {
        Assets {
            gameover_texture: load_texture("assets/sprites/gameover.png")
                .await
                .expect("Failed to load gameover texture"),
            bird_renderer: BirdTextureAtlas::new().await,
            base_texture_atlas: BaseTextureAtlas::new().await,
            background_texture_atlas: BackgroundTextureAtlas::new().await,
            pipe_texture_atlas: Rc::new(PipeTextureAtlas::new().await),
            number_texture_atlas: NumberTextureAtlas::new().await,
        }
    }
}

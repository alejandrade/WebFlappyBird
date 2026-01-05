use macroquad::texture::{FilterMode, Texture2D, load_texture};

pub struct NumberSprites {
    pub digits: [Texture2D; 10],
}

pub struct NumberTextureAtlas {
    pub number_sprites: NumberSprites,
    pub width: f32,
    pub height: f32,
}

impl NumberTextureAtlas {
    pub async fn new() -> NumberTextureAtlas {
        let mut digits = Vec::new();

        for i in 0..10 {
            let texture = Self::get_texture(i).await;
            texture.set_filter(FilterMode::Nearest);
            digits.push(texture);
        }

        let w = digits[0].width();
        let h = digits[0].height();

        // Convert Vec to array
        let digits_array: [Texture2D; 10] = digits.try_into().unwrap();

        NumberTextureAtlas {
            number_sprites: NumberSprites {
                digits: digits_array,
            },
            width: w,
            height: h,
        }
    }

    pub async fn get_texture(digit: usize) -> Texture2D {
        let path = format!("assets/sprites/{}.png", digit);
        load_texture(&path).await.expect("Failed to load texture")
    }
}

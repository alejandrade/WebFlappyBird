use crate::assets::Assets;

pub trait Node {
    fn update(&mut self, dt: f32);
    fn draw(&mut self, assets: &Assets);
}

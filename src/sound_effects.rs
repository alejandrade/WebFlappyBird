use macroquad::audio::{load_sound, play_sound_once, Sound};

pub struct SoundEffects {
    pub death_sound: Sound,
    pub die_sound: Sound,
    pub hit_sound: Sound,
    pub point_sound: Sound,
    pub swoosh_sound: Sound,
    pub wing_sound: Sound,
}

impl SoundEffects {
    pub async fn new() -> Self {
        let death_sound = load_sound("assets/audio/death-sound-trimmed.ogg")
            .await
            .expect("Failed to load death sound");

        let die_sound = load_sound("assets/audio/die-trimmed.ogg")
            .await
            .expect("Failed to load die sound");

        let hit_sound = load_sound("assets/audio/hit-trimmed.ogg")
            .await
            .expect("Failed to load hit sound");

        let point_sound = load_sound("assets/audio/point-trimmed.ogg")
            .await
            .expect("Failed to load point sound");

        let swoosh_sound = load_sound("assets/audio/swoosh-trimmed.ogg")
            .await
            .expect("Failed to load swoosh sound");

        let wing_sound = load_sound("assets/audio/wing-trimmed.ogg")
            .await
            .expect("Failed to load wing sound");

        Self {
            death_sound,
            die_sound,
            hit_sound,
            point_sound,
            swoosh_sound,
            wing_sound,
        }
    }

    pub fn play_death(&self) {
        play_sound_once(&self.death_sound);
    }

    pub fn play_die(&self) {
        play_sound_once(&self.die_sound);
    }

    pub fn play_hit(&self) {
        play_sound_once(&self.hit_sound);
    }

    pub fn play_point(&self) {
        play_sound_once(&self.point_sound);
    }

    pub fn play_swoosh(&self) {
        play_sound_once(&self.swoosh_sound);
    }

    pub fn play_wing(&self) {
        play_sound_once(&self.wing_sound);
    }
}
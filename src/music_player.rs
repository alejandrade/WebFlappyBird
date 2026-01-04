use macroquad::audio::{load_sound, play_sound, set_sound_volume, stop_sound, Sound};
use macroquad::time::get_time;

enum FadeState {
    None,
    FadingOut { next_index: Option<usize> },
    FadingIn,
}

pub struct MusicPlayer {
    sounds: Vec<Sound>,
    current_index: usize,
    is_playing: bool,
    fade_state: FadeState,
    fade_start_time: f64,
    fade_duration: f64,
}

impl MusicPlayer {
    /// Creates a new MusicPlayer by loading all -compressed.ogg files from the given folder
    pub async fn new(folder_path: &str, fade_duration: f64) -> Result<Self, String> {
        println!("[MUSIC] Loading music from: {}", folder_path);

        // Hardcoded music files for WASM compatibility
        let music_files = vec![
            "Hypnotic-Puzzle-compressed.ogg",
            "Hypnotic-Puzzle2-compressed.ogg",
            "Hypnotic-Puzzle3-compressed.ogg",
            "Hypnotic-Puzzle4-compressed.ogg",
        ];

        println!("[MUSIC] Loading {} music files", music_files.len());

        // Load all sounds sequentially
        let mut sounds = Vec::new();
        for filename in music_files {
            let path = format!("{}/{}", folder_path, filename);
            let start = get_time();
            println!("[MUSIC] Loading: {}", filename);
            let sound = load_sound(&path)
                .await
                .map_err(|e| format!("Failed to load sound {}: {}", path, e))?;
            println!("[MUSIC] Loaded {} in {:.2}s", filename, get_time() - start);
            sounds.push(sound);
        }
        Ok(Self {
            sounds,
            current_index: 0,
            is_playing: false,
            fade_state: FadeState::None,
            fade_start_time: 0.0,
            fade_duration,
        })
    }

    /// Start playing the current song (loops indefinitely)
    pub fn play(&mut self) {
        if self.sounds.is_empty() || self.is_playing {
            return;
        }

        play_sound(
            &self.sounds[self.current_index],
            macroquad::audio::PlaySoundParams {
                looped: true,
                volume: 1.0,
            },
        );

        self.is_playing = true;
        self.fade_state = FadeState::None;
    }

    /// Stop playback with fade out
    pub fn stop(&mut self) {
        if self.sounds.is_empty() || !self.is_playing {
            return;
        }


        self.fade_start_time = get_time();
    }

    /// Skip to the next song with crossfade
    pub fn next(&mut self) {
        if self.sounds.is_empty() {
            return;
        }

        let next_idx = (self.current_index + 1) % self.sounds.len();

        if self.is_playing {
            // Start fade out to next song
            self.fade_state = FadeState::FadingOut {
                next_index: Some(next_idx),
            };
            self.fade_start_time = get_time();
        } else {
            // If not playing, just start the next song
            self.start_next_song(next_idx);
        }
    }

    fn start_next_song(&mut self, new_song_idx: usize) {
        stop_sound(&self.sounds[self.current_index]);
        let previous_current_index = self.current_index; // Store the index of the song that just ended
        self.current_index = new_song_idx; // Update to the new song's index

        let is_wrap_around_to_first = new_song_idx == 0 && previous_current_index == self.sounds.len() - 1;

        if is_wrap_around_to_first {
            // "Go back to fast" - play immediately at full volume, no fade-in
            play_sound(
                &self.sounds[self.current_index],
                macroquad::audio::PlaySoundParams {
                    looped: true,
                    volume: 1.0,
                },
            );
            self.fade_state = FadeState::None; // No fade in needed
        } else {
            // Normal fade-in
            play_sound(
                &self.sounds[self.current_index],
                macroquad::audio::PlaySoundParams {
                    looped: true,
                    volume: 0.0, // Start from 0 for fade-in
                },
            );
            self.fade_state = FadeState::FadingIn;
            self.fade_start_time = get_time();
        }
    }

    /// Call this every frame in your game loop to handle fades
    pub fn update(&mut self) {
        if self.sounds.is_empty() || !self.is_playing {
            return;
        }

        // Handle fading
        match &self.fade_state {
            FadeState::FadingOut { next_index } => {
                let fade_progress = (get_time() - self.fade_start_time) / self.fade_duration;
                if fade_progress >= 1.0 {
                    // Fade out complete
                    println!("[MUSIC] Fade out complete, next_index={:?}", next_index);
                    if let Some(next_idx) = *next_index {
                        // Start next song
                        println!("[MUSIC] Fade out complete, starting next song {}", next_idx);
                        self.start_next_song(next_idx);
                    } else {
                        // Just stop
                        println!("[MUSIC] next_index is None, stopping playback");
                        stop_sound(&self.sounds[self.current_index]);
                        self.is_playing = false;
                        self.fade_state = FadeState::None;
                    }
                } else {
                    // Gradually decrease volume
                    let volume = 1.0 - fade_progress;
                    set_sound_volume(&self.sounds[self.current_index], volume as f32);
                }
            }
            FadeState::FadingIn => {
                let fade_progress = (get_time() - self.fade_start_time) / self.fade_duration;
                if fade_progress >= 1.0 {
                    // Fade in complete
                    set_sound_volume(&self.sounds[self.current_index], 1.0);
                    self.fade_state = FadeState::None;
                } else {
                    // Gradually increase volume
                    let volume = fade_progress;
                    set_sound_volume(&self.sounds[self.current_index], volume as f32);
                }
            }
            FadeState::None => {
                // Just playing normally, looping
            }
        }
    }

}

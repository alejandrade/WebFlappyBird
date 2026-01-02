use macroquad::audio::{load_sound, play_sound, set_sound_volume, stop_sound, Sound};
use macroquad::time::get_time;
use std::fs;

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
        println!("[MUSIC] Reading directory: {}", folder_path);

        // Read directory and collect all -compressed.ogg file paths
        let mut entries: Vec<_> = fs::read_dir(folder_path)
            .map_err(|e| format!("Failed to read directory: {}", e))?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .file_name()
                    .and_then(|s| s.to_str())
                    .map(|s| s.ends_with("-compressed.ogg"))
                    .unwrap_or(false)
            })
            .collect();

        // Sort by filename for consistent order
        entries.sort_by_key(|e| e.path());

        if entries.is_empty() {
            return Err("No -compressed.ogg files found in directory".to_string());
        }

        println!("[MUSIC] Found {} compressed OGG files", entries.len());

        // Load all sounds sequentially
        let mut sounds = Vec::new();
        for entry in entries {
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let start = get_time();
            println!("[MUSIC] Loading: {}", filename);
            let sound = load_sound(path.to_str().unwrap())
                .await
                .map_err(|e| format!("Failed to load sound {:?}: {}", path, e))?;
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

        // Start fade out, no next song
        self.fade_state = FadeState::FadingOut { next_index: None };
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
            // If not playing, just move to next
            self.current_index = next_idx;
        }
    }

    fn start_next_song(&mut self, next_idx: usize) {
        stop_sound(&self.sounds[self.current_index]);
        self.current_index = next_idx;

        play_sound(
            &self.sounds[self.current_index],
            macroquad::audio::PlaySoundParams {
                looped: true,
                volume: 0.0,
            },
        );

        self.fade_state = FadeState::FadingIn;
        self.fade_start_time = get_time();
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
                    if let Some(next_idx) = *next_index {
                        // Start next song
                        self.start_next_song(next_idx);
                    } else {
                        // Just stop
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

    /// Get the current song index
    pub fn current_track(&self) -> usize {
        self.current_index
    }

    /// Get total number of tracks
    pub fn track_count(&self) -> usize {
        self.sounds.len()
    }
}

use std::sync::{LazyLock, Mutex};
use macroquad::audio::{self} ;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

use crate::util::Util;

#[derive(EnumIter, AsRefStr)]
pub enum Music {
    MenuMusic = 0,
    Overworld1 = 1,
    Overworld2= 2,
    BossFight = 3
}

pub struct AudioHandler {
    current_song: Option<audio::Sound>,
    songs: Option<Vec<audio::Sound>>,
}

pub static AUDIOSYSTEM: LazyLock<Mutex<AudioHandler>> = LazyLock::new(|| Mutex::new(AudioHandler::new()));

impl AudioHandler {
    
    fn new() -> Self {
        Self {
            current_song: None,
            songs: None
        }
    }

    pub async fn load_songs(&mut self) { 
        let mut songs: Vec<audio::Sound> = Vec::new(); 

        for song in Music::iter() {
            let snake_name = Util::to_snake_case(song.as_ref());
            let file_path = format!("Sounds/Music/{}.wav", snake_name);
            let loaded_song = audio::load_sound(&file_path.to_string()).await.unwrap();
            songs.push(loaded_song);
        }
        self.songs = Some(songs)
    }

    pub fn play_song(&mut self, song: Music) {
        if let Some(current_song) = &self.current_song {
            audio::stop_sound(current_song);
        }

        if let Some(songs) = &self.songs {
            let index = song as usize;
            if let Some(sound) = songs.get(index) {
                audio::play_sound(&sound, audio::PlaySoundParams {looped: true, volume: 1.});
                self.current_song = Some(sound.clone())
            } 
        }
    }
}

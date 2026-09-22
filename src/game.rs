use macroquad::prelude::*;

use crate::{audio_handler::{AUDIOSYSTEM, Music}, display_handler::DISPLAYSYSTEM, input_handler::INPUTSYSTEM};

pub struct Game;

impl Game {
    pub fn new() -> Self {
        Self
    }

    pub async fn run_game(&mut self) {
        AUDIOSYSTEM.lock().unwrap().load_songs().await;
        AUDIOSYSTEM.lock().unwrap().play_song(Music::MenuMusic);

        loop {
            INPUTSYSTEM.lock().unwrap().run_input();
            DISPLAYSYSTEM.lock().unwrap().display_current_ui();
            next_frame().await
        }
    }

    
}
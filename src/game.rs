use std::time::Instant;

use macroquad::prelude::*;

use crate::{audio_handler::{AUDIOSYSTEM, Music}, display_handler::{DISPLAYSYSTEM, Displays}, input_handler::INPUTSYSTEM, ui_code::loading_ui::loading};

pub struct Game;

impl Game {
    pub fn new() -> Self {
        Self
    }

    async fn load_assets(&mut self) {
        let timer = Instant::now();
        AUDIOSYSTEM.lock().unwrap().load_songs().await;
        info!("Load assets took {:?} long", timer.elapsed())
    }

    
    pub async fn run_game(&mut self) {
        let load_time = 400;//Seconds
        for i in 0..load_time {
            loading(i as f32 / (load_time - 1) as f32);
            next_frame().await;
        }
        
        self.load_assets().await;

        DISPLAYSYSTEM.lock().unwrap().change_current_ui(Displays::MainMenu);
        AUDIOSYSTEM.lock().unwrap().play_song(Music::MenuMusic);

        loop {
            INPUTSYSTEM.lock().unwrap().run_input();
            DISPLAYSYSTEM.lock().unwrap().display_current_ui();
            next_frame().await
        }
    }
}
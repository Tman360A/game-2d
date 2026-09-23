use std::sync::{LazyLock, Mutex};
use macroquad::{miniquad::window::quit, prelude::*};

use crate::{audio_handler::{AUDIOSYSTEM, Music}, display_handler::{DISPLAYSYSTEM, Displays}, window_controls::WindowControls};

pub struct InputHandler;

pub static INPUTSYSTEM: LazyLock<Mutex<InputHandler>> = LazyLock::new(|| Mutex::new(InputHandler::new()));

impl InputHandler {

    fn new() -> Self {
        Self
    }

    pub fn run_input(&mut self) {
        if is_key_down(KeyCode::Escape) {
            info!("The Program Has Closed");
            quit();
        }
        if is_key_pressed(KeyCode::F11) {
            info!("F11 Pressed");
            WindowControls::set_fullscreen(!WindowControls::is_fullscreen());
        }
        if is_key_pressed(KeyCode::M) {
            AUDIOSYSTEM.lock().unwrap().play_song(Music::BossFight);
        }
        if is_key_pressed(KeyCode::N) {
        }
    }
}


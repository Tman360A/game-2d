use macroquad::{miniquad::window::quit, prelude::*};

use crate::window_controls::WindowControls;

pub fn input_handler() {
    if is_key_down(KeyCode::Escape) {
        info!("The Program Has Closed");
        quit();
    }
    if is_key_pressed(KeyCode::F11) {
        info!("F11 Pressed");
        WindowControls::set_fullscreen(!WindowControls::is_fullscreen());
    }
}
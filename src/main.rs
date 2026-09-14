mod player;
mod window_controls;
mod input_handler;
mod audio_handler;
mod util;
pub(crate) mod ui_code;

use macroquad::prelude::*;

use crate::{audio_handler::{AudioHandler, Music}, input_handler::input_handler, ui_code::main_menu::main_menu, window_controls::WindowControls};


fn window_conf() -> Conf {
    Conf {
        window_title: "2D Game".to_owned(),
        window_width: 1920,
        window_height: 1080,
        high_dpi: false,
        fullscreen: WindowControls::is_fullscreen(),
        sample_count: 0,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("src/assets");
    let mut audio_handler = AudioHandler::new();
    audio_handler.load_songs().await;
    audio_handler.play_song(Music::MenuMusic);

    loop {
        
        input_handler();
        main_menu();
        next_frame().await
    }
}

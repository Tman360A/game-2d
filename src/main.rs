mod game;
mod display_handler;
mod input_handler;
mod audio_handler;
mod window_controls;
mod util;
pub(crate) mod ui_code;
pub(crate) mod platformer;

use macroquad::prelude::*;

use crate::{game::Game, window_controls::WindowControls};


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

    //Game Setup. This is where everything is put together
    let mut game = Game::new();

    //runs the game until exited
    game.run_game().await;
}

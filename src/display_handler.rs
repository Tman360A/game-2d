use std::sync::{LazyLock, Mutex};
use crate::{platformer::{game_state::GAMESTATE, platformer_display::platformer_display}, ui_code::main_menu_ui::main_menu};

pub enum Displays {
    MainMenu,
    Settings,
    Game,
}

pub struct DisplayHandler {
    current_display: Displays,
}

pub static DISPLAYSYSTEM: LazyLock<Mutex<DisplayHandler>> = LazyLock::new(|| Mutex::new(DisplayHandler::new()));

impl DisplayHandler {
    

    fn new() -> Self {
        Self {
            current_display: Displays::MainMenu,
        }
    }

    pub fn change_current_ui(&mut self, display: Displays) {
        self.current_display = display;
    }

    pub fn display_current_ui(&mut self) -> () {
        match self.current_display {
            Displays::MainMenu => main_menu(self),
            Displays::Game => platformer_display(&mut GAMESTATE.lock().unwrap()),
            _ => (),
        }
    }
}

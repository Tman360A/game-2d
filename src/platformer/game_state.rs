use std::sync::{LazyLock, Mutex};

use macroquad::camera::Camera2D;

use crate::platformer::player::Player;

pub struct GameState {
    pub player: Player,
    pub camera: Camera2D
}

pub static GAMESTATE: LazyLock<Mutex<GameState>> = LazyLock::new(|| Mutex::new(GameState::new()));

impl GameState {
    fn new() -> Self {
        Self {
            player: Player::new(),
            camera: Camera2D {
                ..Default::default()
            } 
        }
    }
}
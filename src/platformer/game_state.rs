use std::sync::{LazyLock, Mutex};

use macroquad::{camera::Camera2D, math::vec2, window::{screen_height, screen_width}};

use crate::platformer::player::Player;

pub struct GameState {
    pub player1: Player,
    pub player2: Player,
    pub camera: Camera2D,
    pub is_following1: bool
}

pub static GAMESTATE: LazyLock<Mutex<GameState>> = LazyLock::new(|| Mutex::new(GameState::new()));

impl GameState {
    fn new() -> Self {
        Self {
            player1: Player::new(),
            player2: Player::new(),
            camera: Camera2D {
                zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
                ..Default::default()
            },
            is_following1: true 
        }
    }
}
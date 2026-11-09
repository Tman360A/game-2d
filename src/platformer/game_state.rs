use std::sync::{LazyLock, Mutex};

use macroquad::{camera::Camera2D, math::{Rect, vec2}, window::{screen_height, screen_width}};

use crate::platformer::player::Player;

pub struct GameState {
    pub player1: Player,
    pub player2: Player,
    pub camera: Camera2D,
    pub is_following1: bool,
    pub level_objects: Vec<Rect>
}

pub static GAMESTATE: LazyLock<Mutex<GameState>> = LazyLock::new(|| Mutex::new(GameState::new()));

impl GameState {

    fn new() -> Self {

        let level_objects: Vec<Rect> = vec![
            Rect::new(-1000., 100., 2000., 50.),
            Rect::new(-800. , -400., 500., 50.),
            Rect::new(600., -200., 600., 50.)
        ];
        Self {
            player1: Player::new(vec2(-200., 0.)),
            player2: Player::new(vec2(200., 0.)),
            camera: Camera2D {
                zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
                ..Default::default()
            },
            is_following1: true,
            level_objects: level_objects 
        }
    }
}
use macroquad::math::{Vec2, vec2};

pub struct Player {
    pub pos: Vec2
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: vec2(0., 0.) 
        }
    }
}
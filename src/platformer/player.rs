use macroquad::math::{Rect, Vec2, vec2};

#[derive(Clone, Copy)]
pub struct Player {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub vel: Vec2,
    pub size: Vec2,
    pub is_grounded: bool
}

impl Player {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos: pos,
            prev_pos: vec2(0., 0.),
            vel: vec2(0., 0.),
            size: vec2(100., 100.),
            is_grounded: true 
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.pos.x , self.pos.y, self.size.x, self.size.y)
    }
}
use macroquad::{prelude::*, ui::Ui};

pub struct CommonUi {
    pub ui_size: Vec2,
    pub button_padding: Vec2
}

impl CommonUi {
    pub fn new(ui_size: Vec2) -> Self {
        Self {
            ui_size,
            button_padding: vec2(20., 10.) }
    }

    pub fn centered_button(&mut self, ui: &mut Ui, label: &str) -> bool {
        let text_size = ui.calc_size(label);
        let x = 10.;
        let y = 20.;
        let clicked = ui.button(vec2(x, y), label);
        clicked   
    }
}
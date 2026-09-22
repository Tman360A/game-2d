use macroquad::miniquad::window::quit;
use macroquad::prelude::*;
use macroquad::ui::{Skin, hash, root_ui};

use crate::ui_code::common_ui::{self, CommonUi};

pub fn main_menu() {
    let ui_size = vec2(1000., 600.);
    let ui_position = (vec2(screen_width(), screen_height()) / 2.)  - (ui_size / 2.);

    //Skin for the UI
    let skin = { 
        let persent = 100.;

        let window_style = root_ui()
        .style_builder()
        .color(DARKGRAY.with_alpha(persent / 180.))
        .build();

        let label_style = root_ui()
            .style_builder()
            .font_size(50)
            .text_color(DARKBLUE)
            .build();

        let button_style = root_ui()
            .style_builder()
            .color(BLACK)
            .color_hovered(DARKBLUE)
            .font_size(40)
            .text_color(WHITE)
            .build();


        Skin {
            window_style,
            label_style,
            button_style,
            ..root_ui().default_skin()
        }
    };

    //Applies the Skin
    root_ui().push_skin(&skin);

    //Displays the UI
    root_ui().window(hash!(), ui_position , ui_size, |ui|{
        let title = "The 2D Platformer Game";
        let text_size = ui.calc_size(title);

        let mut common_ui = CommonUi::new(ui_size);

        ui.label(vec2((ui_size.x - text_size.x) / 2., 0.), title);
        let playSize = ui.calc_size("Play");
        if ui.button(vec2( 500. - playSize.x ,300.), "Play") {

        }

        if common_ui.centered_button(ui, "Settings") {

        }
        if common_ui.centered_button(ui, "Exit") {
            quit();
        }

    });

    root_ui().pop_skin();
}


use macroquad::prelude::*;
use macroquad::ui::{Skin, hash, root_ui};

pub fn loading(progress: f32) {
    let ui_size = vec2(screen_width() / 1.5 , 25.);
    let ui_position = (vec2(screen_width(), screen_height()) / 2.)  - (ui_size / 2.);

    let skin = {

        let window_style = root_ui()
            .style_builder()
            .color(GRAY)
            .build();

        let label_style = root_ui()
            .style_builder()
            .font_size(20)
            .build();

        let progress_bar_style = root_ui()
            .style_builder()
            .color(DARKGRAY)
            .color_hovered(RED)
            .build();

        Skin {
            window_style,
            label_style,
            progress_bar_style,
            ..root_ui().default_skin()
        }
    };

    root_ui().push_skin(&skin);

    root_ui().window(hash!(), ui_position, ui_size, |ui|{
        ui.progress_bar("Loading...", progress);
    });

    root_ui().pop_skin();
}
use macroquad::prelude::*;

use crate::platformer::game_state::GameState;

pub fn platformer_display(state:&mut GameState) {
    let dt = get_frame_time();

    if is_key_down(KeyCode::A) {
        state.player.pos.x -= 10. * dt;
    }

    if is_key_down(KeyCode::D) {
        state.player.pos.x += 10. * dt;
    }
    state.camera.target = state.camera.target.lerp(state.player.pos, 5. * dt);
    set_camera(&state.camera);
    draw_rectangle(state.player.pos.x, state.player.pos.y, 10., 10., BLUE);
    draw_rectangle(0., 0., 1920., 40., RED);
}
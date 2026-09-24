use macroquad::prelude::*;

use crate::platformer::game_state::GameState;

pub fn platformer_display(state:&mut GameState) {
    let dt = get_frame_time();
    let player_speed = 300.;

    let following1 = state.is_following1;

    let mut player_pos: Vec2;
    if state.is_following1 == true {
        player_pos = state.player1.pos;
    } else {
        player_pos = state.player2.pos
    }

    if is_key_down(KeyCode::A) {
        player_pos.x -= player_speed * dt;
    }

    if is_key_down(KeyCode::D) {
        player_pos.x += player_speed * dt;
    }

    if is_key_pressed(KeyCode::Space) {
        state.is_following1 = !state.is_following1
    }

    if following1 {
        state.player1.pos = player_pos;
    } else {
        state.player2.pos = player_pos;
    }
    
    state.camera.target = state.camera.target.lerp(player_pos, 5. * dt);
    set_camera(&state.camera);
    draw_rectangle(0., 100., 1920., 40., RED);
    draw_rectangle(state.player1.pos.x, state.player1.pos.y, 100., 100., BLUE);
    draw_rectangle(state.player2.pos.x, state.player2.pos.y, 100., 100., GREEN);
}
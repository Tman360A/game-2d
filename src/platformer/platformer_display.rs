use macroquad::prelude::*;

use crate::platformer::{game_state::GameState, physics::Physics, player::Player};

const JUMP_FORCE: f32 = -800.; 
const MOVE_SPEED: f32 = 300.;

pub fn platformer_display(state:&mut GameState) {
    let dt = get_frame_time();
    let player_pos;
    {
        let current_player: &mut Player = 
            if state.is_following1 {&mut state.player1}
            else {&mut state.player2};

            current_player.vel.x = 0.0;
            
            if is_key_down(KeyCode::A) {
                current_player.vel.x = -MOVE_SPEED;
            }
            if is_key_down(KeyCode::D) {
                current_player.vel.x = MOVE_SPEED;
            }
            if is_key_down(KeyCode::Space) && current_player.is_grounded {
                current_player.is_grounded = false;
                current_player.vel.y = JUMP_FORCE;
            }

        player_pos = current_player.pos;   
    }
    
    //adds player one to the object vec
    let mut level_objects_player1 = state.level_objects.clone();
    level_objects_player1.push(Rect::new(state.player2.pos.x, state.player2.pos.y, state.player2.size.x, state.player2.size.y));

    //adds player two the object vec
    let mut level_objects_player2 = state.level_objects.clone();
    level_objects_player2.push(Rect::new(state.player1.pos.x, state.player1.pos.y, state.player1.size.x, state.player1.size.y));
    
    Physics::update_universal_physics(&mut state.player1, &level_objects_player1, dt);
    Physics::update_universal_physics(&mut state.player2, &level_objects_player2, dt);
    Physics::update_player_physics(&mut state.player1, &mut state.player2);



    if is_key_pressed(KeyCode::LeftShift) {
        state.is_following1 = !state.is_following1
    }

    state.camera.target = state.camera.target.lerp(vec2(player_pos.x + 50., player_pos.y + 50.), 5. * dt);
    set_camera(&state.camera);
    for o in &state.level_objects {
        draw_rectangle(o.x, o.y, o.w, o.h, RED);
    }
    draw_rectangle(state.player1.pos.x, state.player1.pos.y, 100., 100., BLUE);
    draw_rectangle(state.player2.pos.x, state.player2.pos.y, 100., 100., GREEN);
}
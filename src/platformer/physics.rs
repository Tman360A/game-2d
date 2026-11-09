use macroquad::prelude::*;

use crate::platformer::player::Player;

pub struct Physics;

const GRAVITY: f32 = 1200.;
const DRAG: f32 = 100.;

impl Physics {

    pub fn update_universal_physics(player: &mut Player, platforms: &[Rect], dt: f32) {
        player.prev_pos = player.pos;

        //adds gravity to player
        player.vel.y += GRAVITY * dt;
        
        //adds drag to the horizontal movement
        if player.vel.x > 0. {
            player.vel.x -= DRAG * dt;
        } else if player.vel.x < 0. {
            player.vel.x += DRAG * dt;
        }

        //adds terminal velocity
        if player.vel.y >= 800. {
            player.vel.y = 800.
        }

        //stops player when grounded
        if player.is_grounded {
            player.vel.y = 0.
        }

        //updates player position based off of velocity
        player.pos.x += player.vel.x * dt;
        player.pos.y += player.vel.y * dt;


        //Y axis physics for the player
        player.is_grounded = false;
        for platform in platforms {
            if player.rect().overlaps(platform) {
                let was_above = player.prev_pos.y + player.size.y <= platform.y;
                let was_below = player.prev_pos.y >= platform.y + platform.h;

                if was_above {
                    player.pos.y = platform.y - player.size.y;
                    player.vel.y = 0.;
                    player.is_grounded = true; 
                } else if was_below {
                    player.pos.y = platform.y + platform.h;
                    player.vel.y = 0.;
                } 
            }
        }

        //X axis physics for the player
        for platform in platforms {
            if player.rect().overlaps(platform) {
                let was_left = player.prev_pos.x + player.size.x <= platform.x;
                let was_right = player.prev_pos.x >= platform.x + platform.w;

                if was_left {
                    player.pos.x = platform.x - player.size.x; // snap flush to the left side
                    player.vel.x = 0.;
                } else if was_right {
                    player.pos.x = platform.x + platform.w; // snap flush to the right side
                    player.vel.x = 0.;
                }
            }
        }
    }

    pub fn update_player_physics(player1: &mut Player, player2: &mut Player) {
        if player1.rect().overlaps(&player2.rect()) {
            player1.pos = player1.prev_pos;
        }
    }
}
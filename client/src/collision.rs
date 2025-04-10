use crate::common::{Obstacle, Block, Hook, Player, PlayerAttach, MAP_CONFIG, OBSTACLE_CONFIG, BLOCK_CONFIG, PLAYER_CONFIG};
use crate::db_connection::{update_player_position, CtxWrapper};
use crate::collision::check_collision;
use bevy::math::Vec2 as BevyVec2;
use crate::module_bindings::Vec2 as DBVec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;


pub fn check_collision<T: Component>(
    new_pos: Vec2,
    targets: &Query<&Transform, With<T>>,
    target_size: Vec2,
) -> bool {
    let player_radius = PLAYER_CONFIG.size.x.min(PLAYER_CONFIG.size.y) / 2.0;
    let target_radius = target_size.x.min(target_size.y) / 2.0;
    let collision_distance = player_radius + target_radius;

    targets
        .iter()
        .any(|transform| new_pos.distance(transform.translation.truncate()) < collision_distance)
}
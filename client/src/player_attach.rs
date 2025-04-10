use bevy::prelude::*;

use crate::common::{
    PlayerAttach, 
    AttachedBlock,
    PlayerGrid,
    Player, 
    Hook, 
    PLAYER_CONFIG,
};


pub fn attach_objects(
    player_query: Query<(&Transform, &PlayerGrid), With<Player>>,
    mut objects_query: Query<(&AttachedBlock, &mut Transform), Without<Player>>,
) {
    //if let Ok(player_transform) = player_query.get_single() {
    for (player_transform, player_grid) in player_query.iter() {
        for (attach, mut transform) in objects_query.iter_mut() {
            // Calculate the rotated offset
            let rotated_offset = player_transform.rotation
                * Vec3::new(
                    attach.grid_offset.0 as f32 * player_grid.cell_size,
                    attach.grid_offset.1 as f32 * player_grid.cell_size,
                    5.0,
                );

            // Update position and rotation
            transform.translation = player_transform.translation + rotated_offset;
            transform.rotation = player_transform.rotation;
        }
    }
}
pub fn attach_items(
    player_query: Query<(&Transform, &PlayerGrid), With<Player>>,
    mut items_query: Query<(&PlayerAttach, &mut Transform), Without<Player>>,
) {
    //if let Ok(player_transform) = player_query.get_single() {
    for (player_transform, player_grid) in player_query.iter() {
        for (attach, mut transform) in items_query.iter_mut() {
            // Calculate the rotated offset
            let rotated_offset = player_transform.rotation
                * Vec3::new(attach.offset.x as f32, attach.offset.y as f32, 5.0);

            // Update position and rotation
            transform.translation = player_transform.translation + rotated_offset;
            transform.rotation = player_transform.rotation;
        }
    }
}

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

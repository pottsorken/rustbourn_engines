use bevy::prelude::*;

use crate::common::{AttachedBlock, Hook, Player, PlayerAttach, PlayerGrid};

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


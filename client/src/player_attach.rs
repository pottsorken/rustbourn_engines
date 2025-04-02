use bevy::prelude::*;

use crate::player::{Player,
    setup_player,
    player_movement};

// use crate::hook::{Hook}

#[derive(Component)]
pub struct PlayerAttach {
    pub offset: Vec2,
}

pub fn attatch_objects(
    player_query: Query<&Transform, With<Player>>,
    mut objects_query: Query<(&PlayerAttach, &mut Transform), Without<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (attach, mut transform) in objects_query.iter_mut() {
            // Calculate the rotated offset
            let rotated_offset = player_transform.rotation * Vec3::new(attach.offset.x, attach.offset.y, 5.0);
            
            // Update position and rotation
            transform.translation = player_transform.translation + rotated_offset;
            transform.rotation = player_transform.rotation;
        }
    }
}
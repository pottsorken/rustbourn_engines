use bevy::prelude::*;

use crate::common::{PlayerAttach, Player,Hook};


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
pub fn attatch_objects_hook(
    hook_query: Query<&Transform, With<Hook>>,
    mut objects_query: Query<(&PlayerAttach, &mut Transform), Without<Hook>>,
) {
    if let Ok(hook_transform) = hook_query.get_single() {
        for (attach, mut transform) in objects_query.iter_mut() {
            // Ensure this object is meant to attach to the Hook
            if attach.target == "Hook" {
                // Calculate the rotated offset
                let rotated_offset = hook_transform.rotation * Vec3::new(attach.offset.x, attach.offset.y, 5.0);

                // Update position and rotation to follow the Hook
                transform.translation = hook_transform.translation + rotated_offset;
                transform.rotation = hook_transform.rotation;
            }
        }
    }
}
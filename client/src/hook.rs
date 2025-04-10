use crate::common::{
    AttachedBlock, Block, Hook, Obstacle, Player, PlayerAttach, PlayerGrid, BLOCK_CONFIG,
    PLAYER_CONFIG,
};
use crate::grid::increment_grid_pos;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn setup_hook(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a player sprite at position (0, 0) at a higher z-index than map
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(25.0, 26.0)), // Square size 100x100 pixels
            image: asset_server.load("sprites/GrapplingHook.png"),
            //color:Color::srgb(0.8, 0.4, 0.2),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 8.0),
        Hook {
            hook_speed: 500.0,
            hook_max_range: 100.0,
        },
        PlayerAttach {
            offset: Vec2::new(0.0, 20.0), // Offset from player's center
        },
    ));
}

pub fn hook_controls_short(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Sprite, &mut Transform), With<Hook>>,
    time: Res<Time>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {}
}

// fn extend_rope(mut query: Query<&mut Transform, With<Hook>>) {
//     for mut transform in query.iter_mut() {
//         transform.scale.y += 5.0;
//     }
// }

pub fn hook_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut hook_query: Query<
        (&mut Sprite, &mut Transform, &Hook),
        (With<Hook>, Without<Obstacle>, Without<Block>),
    >,
    block_query: Query<(Entity, &Transform), (With<Block>, Without<AttachedBlock>)>,
    mut player_query: Query<
        (Entity, &Transform, &mut Player, &mut PlayerGrid),
        (Without<Obstacle>, Without<Block>, Without<Hook>),
    >,
    attachable_blocks: Query<&PlayerAttach>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut sprite, mut transform, hook) in hook_query.iter_mut() {
        let growth_rate = hook.hook_speed;
        let growth_amount = growth_rate * time.delta_secs();

        if let Some(size) = sprite.custom_size {
            let mut new_height = size.y;
            let mut y_offset = 0.0;

            if keyboard_input.pressed(KeyCode::Space) {
                if size.y < hook.hook_max_range {
                    new_height = (size.y + growth_amount).min(hook.hook_max_range);
                    y_offset = (new_height - size.y) / 2.0;
                }
            } else {
                if size.y > 0.0 {
                    new_height = (size.y - growth_amount).max(0.0);
                    y_offset = -(size.y - new_height) / 2.0;
                }
            }

            sprite.custom_size = Some(Vec2::new(size.x, new_height));
            transform.translation.y += y_offset;

            // Hook tip position
            let hook_tip =
                transform.translation + transform.rotation * Vec3::new(0.0, new_height, 0.0);

            if let Ok((_player_entity, player_transform, mut player, mut grid)) =
                player_query.get_single_mut()
            {
                let mut collided_with_block = false;

                for (block_entity, block_transform) in block_query.iter() {
                    let block_radius = BLOCK_CONFIG.size.x.min(BLOCK_CONFIG.size.y) / 2.0;
                    let hook_radius = 5.0; // Hook tip radius
                    let collision_distance = block_radius + hook_radius;

                    if hook_tip
                        .truncate()
                        .distance(block_transform.translation.truncate())
                        < collision_distance
                    {
                        // Check if block already attached
                        if grid.load < grid.capacity
                            && attachable_blocks.get(block_entity).is_err()
                            && player.block_count < PLAYER_CONFIG.max_block_count
                        {
                            let nextpos = grid.next_free_pos.clone();
                            commands.entity(block_entity).insert(AttachedBlock {
                                grid_offset: nextpos, // WARN: subject to change constants
                                player_entity: _player_entity,
                            });
                            grid.block_position.insert(nextpos, block_entity);
                            println!(
                                "Attach at gridpos ({}, {})",
                                grid.next_free_pos.0, grid.next_free_pos.1
                            );

                            // increment grid pos
                            increment_grid_pos(&mut grid);

                            player.block_count += 1;
                        }
                    }
                }
            }
        }
    }
}

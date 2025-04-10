use crate::common::{
    AttachedBlock, Block, Hook, HookCharge, Obstacle, Player, PlayerAttach, PlayerGrid, BLOCK_CONFIG,
    PLAYER_CONFIG,
};
use bevy::prelude::*;
use std::collections::HashMap;

pub fn setup_hook(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(25.0, 0.0)),
            image: asset_server.load("sprites/GrapplingHook.png"),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 8.0),
        Hook {
            hook_speed: 500.0,
            hook_max_range: 400.0,
        },
        HookCharge { 
            time_held: 0.0, 
            target_length: 0.0,
        },
        PlayerAttach {
            offset: Vec2::new(0.0, 20.0),
        },
    ));
}

pub fn hook_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut hook_query: Query<(&mut Sprite, &mut Transform, &Hook, &mut HookCharge), With<Hook>>,
    time: Res<Time>,
) {
    for (mut sprite, mut transform, hook, mut charge) in hook_query.iter_mut() {
        let extend_speed = 500.0;
        let retract_speed = hook.hook_speed;

        if keyboard_input.pressed(KeyCode::Space) && charge.target_length == 0.0 {
                charge.time_held += time.delta_secs();
        }

        if keyboard_input.just_released(KeyCode::Space) {
            // Calculate target length
            charge.target_length = (charge.time_held/2.0 * hook.hook_speed).min(hook.hook_max_range);
            charge.time_held = 0.0;
        }

        let current_height = sprite.custom_size.unwrap().y;

        if charge.target_length > 0.0 {
            // Animate extend
            let next_height = (current_height + extend_speed * time.delta_secs())
                .min(charge.target_length);

            transform.translation.y += (next_height - current_height) / 2.0;
            sprite.custom_size = Some(Vec2::new(sprite.custom_size.unwrap().x, next_height));

            if (next_height - charge.target_length).abs() < 0.1 {
                charge.target_length = 0.0; // Reached target length
            }
        } else if current_height > 0.0 && !keyboard_input.pressed(KeyCode::Space) {
            // Retract when idle
            let next_height = (current_height - retract_speed * time.delta_secs()).max(0.0);
            transform.translation.y -= (current_height - next_height) / 2.0;
            sprite.custom_size = Some(Vec2::new(sprite.custom_size.unwrap().x, next_height));
        }
    }
}



pub fn hook_collision_system(
    hook_query: Query<(&Transform, &Sprite), With<Hook>>,
    block_query: Query<(Entity, &Transform), (With<Block>, Without<AttachedBlock>)>,
    mut player_query: Query<(Entity, &Transform, &mut Player, &mut PlayerGrid), Without<Hook>>,
    attachable_blocks: Query<&PlayerAttach>,
    mut commands: Commands,
) {
    let (hook_transform, sprite) = if let Ok(val) = hook_query.get_single() {
        val
    } else {
        return;
    };

    let hook_tip = hook_transform.translation + hook_transform.rotation * Vec3::new(0.0, sprite.custom_size.unwrap().y, 0.0);

    if let Ok((player_entity, player_transform, mut player, mut grid)) = player_query.get_single_mut() {
        for (block_entity, block_transform) in block_query.iter() {
            let block_radius = BLOCK_CONFIG.size.x.min(BLOCK_CONFIG.size.y) / 2.0;
            let hook_radius = 5.0;
            let collision_distance = block_radius + hook_radius;

            if hook_tip.truncate().distance(block_transform.translation.truncate()) < collision_distance {
                if attachable_blocks.get(block_entity).is_err() && player.block_count < PLAYER_CONFIG.max_block_count {
                    let nextpos = grid.next_free_pos;
                    commands.entity(block_entity).insert(AttachedBlock {
                        grid_offset: nextpos,
                        player_entity,
                    });
                    grid.block_position.insert(nextpos, block_entity);

                    grid.next_free_pos.0 += 1;
                    if grid.next_free_pos == (0, 0) {
                        grid.next_free_pos.0 += 1;
                    }
                    if grid.next_free_pos.0 > grid.grid_size.0 {
                        grid.next_free_pos.0 = -grid.grid_size.0;
                        grid.next_free_pos.1 -= 1;
                    }

                    player.block_count += 1;
                }
            }
        }
    }
}

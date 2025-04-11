use crate::common::{
    AttachedBlock, Block, Hook, HookCharge, Obstacle, Player, PlayerAttach, PlayerGrid,
    BLOCK_CONFIG, HOOK_CONFIG, PLAYER_CONFIG,
};
use crate::grid::increment_grid_pos;
use bevy::prelude::*;

pub fn setup_hook(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            custom_size: Some(HOOK_CONFIG.hook_size),
            image: asset_server.load(HOOK_CONFIG.hook_path),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 8.0),
        Hook {
            hook_speed: HOOK_CONFIG.hook_speed,
            hook_max_range: HOOK_CONFIG.hook_max_range,
        },
        HookCharge {
            time_held: 0.0,
            target_length: 0.0,
        },
        PlayerAttach {
            offset: HOOK_CONFIG.player_attach_offset,
        },
    ));
}

pub fn hook_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut hook_query: Query<
        (&mut Sprite, &mut Transform, &Hook, &mut HookCharge),
        (With<Hook>, Without<Obstacle>, Without<Block>),
    >,
    mut block_query: Query<
        (Entity, &Transform, Option<&mut AttachedBlock>),
        (With<Block>, Without<Player>),
    >,
    mut player_query: Query<
        (Entity, &Transform, &mut Player, &mut PlayerGrid),
        (Without<Obstacle>, Without<Block>, Without<Hook>),
    >,
    attachable_blocks: Query<&PlayerAttach>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut sprite, mut transform, hook, mut charge) in hook_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) && charge.target_length == 0.0 {
            charge.time_held += time.delta_secs();
        }

        if keyboard_input.just_released(KeyCode::Space) {
            charge.target_length =
                (charge.time_held / 2.0 * hook.hook_speed).min(hook.hook_max_range);
            charge.time_held = 0.0;
        }

        let current_height = sprite.custom_size.unwrap().y;

        if charge.target_length > 0.0 {
            let next_height = (current_height + HOOK_CONFIG.extend_speed * time.delta_secs())
                .min(charge.target_length);

            transform.translation.y += (next_height - current_height) / 2.0;
            sprite.custom_size = Some(Vec2::new(sprite.custom_size.unwrap().x, next_height));

            if (next_height - charge.target_length).abs() < 0.1 {
                charge.target_length = 0.0;
            }
        } else if current_height > 0.0 && !keyboard_input.pressed(KeyCode::Space) {
            let next_height =
                (current_height - HOOK_CONFIG.retract_speed * time.delta_secs()).max(0.0);
            transform.translation.y -= (current_height - next_height) / 2.0;
            sprite.custom_size = Some(Vec2::new(sprite.custom_size.unwrap().x, next_height));
        }
    }
}

pub fn hook_collision_system(
    hook_query: Query<(&Transform, &Sprite), With<Hook>>,
    mut block_query: Query<(Entity, &Transform, Option<&mut AttachedBlock>), (With<Block>)>,
    mut player_query: Query<(Entity, &Transform, &mut Player, &mut PlayerGrid), Without<Hook>>,
    attachable_blocks: Query<&PlayerAttach>,
    mut commands: Commands,
) {
    let (hook_transform, sprite) = if let Ok(val) = hook_query.get_single() {
        val
    } else {
        return;
    };

    let hook_tip = hook_transform.translation
        + hook_transform.rotation * Vec3::new(0.0, sprite.custom_size.unwrap().y, 0.0);

    if let Ok((player_entity, _player_transform, mut player, mut grid)) =
        player_query.get_single_mut()
    {
        for (block_entity, block_transform, mut attach_link_option) in block_query.iter_mut() {
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

                    // NOTE: If block is attached or not. (Option<AttachedBlock>)
                    if let Some(mut attach_link) = attach_link_option {
                        // TODO: Remove entity from previous owner hashmap
                        attach_link.player_entity = player_entity;
                        attach_link.grid_offset = nextpos;
                    } else {
                        commands.entity(block_entity).insert(AttachedBlock {
                            grid_offset: nextpos,
                            player_entity: player_entity,
                        });
                    }
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

use crate::module_bindings::*;
use crate::{
    common::{
        AttachedBlock, Block, Hook, HookCharge, Obstacle, OpponentHook, Player, PlayerAttach,
        PlayerGrid, BLOCK_CONFIG, HOOK_CONFIG, OBSTACLE_CONFIG, PLAYER_CONFIG, CtxWrapper,
    },
    db_connection::load_obstacles,
    grid::increment_grid_pos,
    opponent,
};
use bevy::{prelude::*, transform};
use spacetimedb_sdk::{
    credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey,
};

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
    ctx: Res<CtxWrapper>,
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
            sprite.custom_size = Some(bevy::prelude::Vec2::new(
                sprite.custom_size.unwrap().x,
                next_height,
            ));

            let old_size = sprite.custom_size.unwrap().clone();

            ctx.ctx
                .reducers()
                .update_hook_movement(ctx.ctx.identity(), old_size.x, next_height)
                .unwrap();

            if (next_height - charge.target_length).abs() < 0.1 {
                charge.target_length = 0.0;
            }
        } else if current_height > 0.0 && !keyboard_input.pressed(KeyCode::Space) {
            let next_height =
                (current_height - HOOK_CONFIG.retract_speed * time.delta_secs()).max(0.0);
            transform.translation.y -= (current_height - next_height) / 2.0;

            let old_size = sprite.custom_size.unwrap().clone();

            sprite.custom_size = Some(bevy::prelude::Vec2::new(
                sprite.custom_size.unwrap().x,
                next_height,
            ));

            ctx.ctx
                .reducers()
                .update_hook_movement(ctx.ctx.identity(), old_size.x, next_height)
                .unwrap();
        }
    }
}

pub fn hook_collision_system(
    hook_query: Query<(&Transform, &Sprite), (With<Hook>, Without<Block>)>,
    mut block_query: Query<(Entity, &mut Transform, Option<&mut AttachedBlock>), (With<Block>)>,
    mut player_query: Query<
        (Entity, &Transform, &mut Player, &mut PlayerGrid),
        (Without<Hook>, Without<Block>),
    >,
    attachable_blocks: Query<&PlayerAttach>,
    mut commands: Commands,
) {
    let (hook_transform, sprite) = if let Ok(val) = hook_query.get_single() {
        val
    } else {
        return;
    };

    let hook_tip = hook_transform.translation
        + hook_transform.rotation
            * bevy::prelude::Vec3::new(0.0, sprite.custom_size.unwrap().y, 0.0);

    if let Ok((player_entity, _player_transform, mut player, mut grid)) =
        player_query.get_single_mut()
    {
        for (block_entity, mut block_transform, mut attach_link_option) in block_query.iter_mut() {
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
                    block_transform.translation.x += 200_000.;
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
pub fn spawn_opponent_hook(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    existing_hooks_query: &Query<&OpponentHook>,
    opponent_id: &Identity,
    local_player_id: &Identity,
    x: f32,
    y: f32,
) {
    // Don't spawn hook for yourself
    if opponent_id == local_player_id {
        return;
    }

    // Don't spawn already existing hooks
    for hook in existing_hooks_query.iter() {
        if hook.id == *opponent_id {
            return; // Hook already exists
        }
    }

    commands.spawn((
        Sprite {
            custom_size: Some(HOOK_CONFIG.hook_size),
            image: asset_server.load(HOOK_CONFIG.hook_path),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(x, y, 5.0),
        OpponentHook { id: *opponent_id },
    ));
}

pub fn update_opponent_hook(
    query: &mut Query<(&mut Sprite, &mut Transform, &OpponentHook), With<OpponentHook>>,
    id: &Identity,
    x: f32,
    y: f32,
    rotation: f32,
    width: f32,
    height: f32,
) {
    for (mut sprite, mut transform, hook) in query.iter_mut() {
        if hook.id == *id {
            transform.translation.x = x;
            transform.translation.y = y;
            transform.rotation = Quat::from_rotation_z(rotation).normalize();
            sprite.custom_size = Some(bevy::prelude::Vec2::new(width, height));
        }
    }
}

pub fn despawn_opponent_hooks(
    mut commands: Commands,
    ctx_wrapper: Res<CtxWrapper>,
    query: Query<(Entity, &OpponentHook)>,
) {
    // List all online players
    let online_players: Vec<Identity> = ctx_wrapper
        .ctx
        .db
        .player()
        .iter()
        .map(|player| player.identity)
        .collect();

    for (entity, hook) in query.iter() {
        if !online_players.contains(&hook.id) {
            commands.entity(entity).despawn();
        }
    }
}

pub fn handle_obstacle_hit(
    ctx_wrapper: Res<CtxWrapper>,
    hook_query: Query<(&Transform, &Sprite), With<Hook>>,
    obstacle_query: Query<(&Obstacle, &Transform)>,
) {
    let obstacle_radius = OBSTACLE_CONFIG.size.x.min(OBSTACLE_CONFIG.size.y) / 2.0;
    let hook_radius = 6.0;

    // Ensure hook_query and obstacle_query contain valid entities
    if hook_query.is_empty() || obstacle_query.is_empty() {
        return; // Skip if no hooks or obstacles exist
    }

    for (hook_transform, hook_sprite) in &hook_query {
        let hook_tip =
            hook_transform.translation + hook_transform.up() * (hook_sprite.custom_size.unwrap().y); // tip = base + height

        for (obstacle, obstacle_transform) in &obstacle_query {
            let obstacle_pos = obstacle_transform.translation.truncate();
            let obstacle_pos_3d = bevy::prelude::Vec3::new(obstacle_pos.x, obstacle_pos.y, 0.0);
            let distance = hook_tip.distance(obstacle_pos_3d);

            if distance < (hook_radius + obstacle_radius) {
                // Ask SpaceTimeDB to handle the damage
                let _ = ctx_wrapper.ctx.reducers.damage_obstacle(obstacle.id, 1);
            }
        }
    }
}

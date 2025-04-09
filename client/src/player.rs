use crate::common::{
    AttachedBlock, Block, Obstacle, Player, PlayerGrid, BLOCK_CONFIG, MAP_CONFIG, OBSTACLE_CONFIG,
    PLAYER_CONFIG,
};
use crate::db_connection::{update_player_position, CtxWrapper};
use crate::module_bindings::Vec2 as DBVec2;
use crate::player_attach::*;
use bevy::math::Vec2 as BevyVec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::HashMap;

// server
use spacetimedb_sdk::{
    credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey,
};

pub fn setup_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let _window = window_query.get_single().unwrap();

    // Spawn a player sprite at position (0, 0) at a higher z-index than map
    commands.spawn((
        Sprite {
            custom_size: Some(PLAYER_CONFIG.size), // Square size 100x100 pixels
            image: asset_server.load(PLAYER_CONFIG.path),
            ..default()
        },
        //TextureAtlas {
        //    layout: asset_server.load("sprites/top-view/robot_3Dblue.png"),
        //    index: 0,
        //}, -- NOTE: If asset-chart is ever used
        Transform::from_xyz(0.0, 0.0, 2.0),
        Player {
            movement_speed: PLAYER_CONFIG.movement_speed, // meters per second
            rotation_speed: PLAYER_CONFIG.rotation_speed, // degrees per second
            max_block_count: 0,
        },
        PlayerGrid {
            block_position: HashMap::new(),
            grid_size: (1, 1),
            cell_size: 84.,
            next_free_pos: (-1, -1),
        },
    ));
}

pub fn attach_block(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut block_query: Query<(Entity, &mut Transform), (With<Block>, Without<AttachedBlock>)>,
    mut player_query: Query<
        (Entity, &Transform, &mut Player, &mut PlayerGrid),
        (Without<Obstacle>, Without<Block>),
    >,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (player_entity, transform, mut player, mut grid) in &mut player_query {
        let mut rotation_dir = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            rotation_dir += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            rotation_dir -= 1.0;
        }

        // Handle movement with W/S keys (forward/backward relative to rotation)
        let mut move_dir = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            move_dir.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            move_dir.y -= 1.0;
        }

        // Apply movement relative to player's rotation
        if move_dir != Vec3::ZERO {
            let move_direction = transform.rotation * move_dir.normalize();
            let new_pos =
                transform.translation + move_direction * player.movement_speed * time.delta_secs();

            // NOTE: Block collision here
            let mut collided_with_block = false;
            for (block_entity, mut block_transform) in block_query.iter_mut() {
                let block_radius = BLOCK_CONFIG.size.x.min(BLOCK_CONFIG.size.y) / 2.0;
                let player_radius = PLAYER_CONFIG.size.x.min(PLAYER_CONFIG.size.y) / 2.0;
                let collision_distance = block_radius + player_radius;

                if new_pos
                    .truncate()
                    .distance(block_transform.translation.truncate())
                    < collision_distance
                {
                    collided_with_block = true;

                    let nextpos = grid.next_free_pos.clone();
                    commands.entity(block_entity).insert(AttachedBlock {
                        grid_offset: nextpos, // WARN: subject to change constants
                        player_entity,
                    });
                    grid.block_position.insert(nextpos, block_entity);
                    println!(
                        "Attach at gridpos ({}, {})",
                        grid.next_free_pos.0, grid.next_free_pos.1
                    );

                    // increment grid pos
                    grid.next_free_pos.0 += 1;
                    if grid.next_free_pos == (0, 0) {
                        grid.next_free_pos.0 += 1;
                    }
                    if grid.next_free_pos.0 > grid.grid_size.0 {
                        grid.next_free_pos.0 = -grid.grid_size.0;
                        grid.next_free_pos.1 += 1;
                    }

                    //block_transform.translation.x += 300.;
                    //block_transform.translation.y += 300.;
                    // Check if already attached
                    //if attachable_blocks.get(block_entity).is_err() && player.max_block_count < 2 {
                    //let offset = block_transform.translation - transform.translation;

                    //player.max_block_count += 1;
                    //}
                }
            }
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut block_query: Query<(Entity, &Transform), (With<Block>, Without<AttachedBlock>)>,
    attached_block_query: Query<(Entity, &Transform, &AttachedBlock), With<Block>>,
    mut player_query: Query<
        (Entity, &mut Transform, &mut Player, &mut PlayerGrid),
        (Without<Obstacle>, Without<Block>),
    >,
    obstacle_query: Query<&Transform, With<Obstacle>>,
    //attachable_blocks: Query<&PlayerAttach>,
    mut commands: Commands,
    time: Res<Time>,
    ctx: Res<CtxWrapper>,
) {
    //if let Ok((mut transform, _player)) = query.get_single_mut() { // NOTE: merge conflict
    let ctx_wrapper = &ctx.into_inner();

    for (player_entity, mut transform, mut player, mut grid) in &mut player_query {
        // Handle rotation with A/D keys
        let mut rotation_dir = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            rotation_dir += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            rotation_dir -= 1.0;
        }

        // Apply rotation
        if rotation_dir != 0.0 {
            transform.rotate_z(rotation_dir * PLAYER_CONFIG.rotation_speed * time.delta_secs());
        }

        // Handle movement with W/S keys (forward/backward relative to rotation)
        let mut move_dir = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            move_dir.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            move_dir.y -= 1.0;
        }

        // Apply movement relative to player's rotation
        if move_dir != Vec3::ZERO {
            //|| rotation_dir != 0. {
            let mut move_direction = Vec3::ZERO.clone();
            //if move_dir != Vec3::ZERO {
            move_direction = transform.rotation * move_dir.normalize();
            //}
            let new_pos =
                transform.translation + move_direction * player.movement_speed * time.delta_secs();

            let mut collided_with_obstacle =
                check_collision(new_pos.truncate(), &obstacle_query, OBSTACLE_CONFIG.size);

            let mut blocks_collided_obstacles = false;

            // Check collision for all attached blocks
            for (attached_block_entity, attached_block_transform, attached_block_link) in
                attached_block_query.iter()
            {
                if attached_block_link.player_entity == player_entity {
                    let rotated_offset = transform.rotation
                        * Vec3::new(
                            attached_block_link.grid_offset.0 as f32 * grid.cell_size,
                            attached_block_link.grid_offset.1 as f32 * grid.cell_size,
                            5.0,
                        );
                    //* rotation_dir
                    //* player.rotation_speed
                    //* time.delta_secs();

                    let new_block_pos = (attached_block_transform.translation
                        + move_direction * player.movement_speed * time.delta_secs());
                    //+ (rotated_offset);
                    //
                    //* rotation_dir
                    //* player.rotation_speed
                    //* time.delta_secs());

                    blocks_collided_obstacles = check_collision(
                        new_block_pos.truncate(),
                        &obstacle_query,
                        OBSTACLE_CONFIG.size,
                    );
                    if blocks_collided_obstacles {
                        break;
                    }
                }
            }

            let mut collided_with_block = false;

            // NOTE: Block collision logic here
            for (block_entity, mut block_transform) in block_query.iter_mut() {
                let block_radius = BLOCK_CONFIG.size.x.min(BLOCK_CONFIG.size.y) / 2.0;
                let player_radius = PLAYER_CONFIG.size.x.min(PLAYER_CONFIG.size.y) / 2.0;
                let collision_distance = block_radius + player_radius;

                if new_pos
                    .truncate()
                    .distance(block_transform.translation.truncate())
                    < collision_distance
                {
                    collided_with_block = true;
                }
            }

            //println!(" ");
            if !collided_with_obstacle && !collided_with_block && !blocks_collided_obstacles {
                // Apply tanslation
                transform.translation = new_pos;
            }
        }
        update_player_position(ctx_wrapper, &transform);
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let _window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_CONFIG.size / 2.0;

        let world_map_size = bevy::prelude::Vec2::new(
            MAP_CONFIG.map_size.x as f32 * MAP_CONFIG.tile_size.x,
            MAP_CONFIG.map_size.y as f32 * MAP_CONFIG.tile_size.y,
        );
        let half_map_size = world_map_size / 2.0;

        let x_min: f32 = -half_map_size.x + half_player_size.x;
        let x_max: f32 = half_map_size.x - half_player_size.x;
        let y_min: f32 = -half_map_size.y + half_player_size.y;
        let y_max: f32 = half_map_size.y - half_player_size.y;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
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

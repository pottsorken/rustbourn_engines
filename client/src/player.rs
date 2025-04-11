use crate::common::{
    AttachedBlock, Block, Obstacle, Opponent, Player, PlayerGrid, BLOCK_CONFIG, GRID_CONFIG,
    MAP_CONFIG, OBSTACLE_CONFIG, PLAYER_CONFIG,
};
use crate::db_connection::{update_player_position, CtxWrapper};
use crate::module_bindings::*;
use crate::player_attach::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::HashMap;

use rand::Rng;

// server
use spacetimedb_sdk::{
    credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey,
};

pub fn setup_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    ctx: Res<CtxWrapper>,
) {
    let _window = window_query.get_single().unwrap();
    let random_position = generate_random_spawnpoint(ctx.into_inner());

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
        Transform::from_xyz(random_position.0, random_position.1, 2.0),
        Player {
            movement_speed: PLAYER_CONFIG.movement_speed, // meters per second
            rotation_speed: PLAYER_CONFIG.rotation_speed, // degrees per second
            block_count: 0,
        },
        PlayerGrid {
            block_position: HashMap::new(),
            grid_size: GRID_CONFIG.grid_size,
            cell_size: GRID_CONFIG.cell_size,
            next_free_pos: GRID_CONFIG.next_free_pos,
            capacity: GRID_CONFIG.capacity,
            load: GRID_CONFIG.load,
        },
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut block_query: Query<(Entity, &Transform), (With<Block>, Without<AttachedBlock>)>,
    attached_block_query: Query<(Entity, &Transform, &AttachedBlock), With<Block>>,
    mut player_query: Query<
        (Entity, &mut Transform, &mut Player, &mut PlayerGrid),
        (Without<Obstacle>, Without<Block>, Without<Opponent>),
    >,
    opponent_query: Query<&Transform, With<Opponent>>,
    obstacle_query: Query<&Transform, With<Obstacle>>,
    //attachable_blocks: Query<&PlayerAttach>,
    mut _commands: Commands,
    time: Res<Time>,
    ctx: Res<CtxWrapper>,
) {
    //if let Ok((mut transform, _player)) = query.get_single_mut() { // NOTE: merge conflict
    let ctx_wrapper = &ctx.into_inner();

    let opponent_transforms: Vec<Transform> = opponent_query.iter().cloned().collect();
    for (player_entity, mut transform, player, grid) in &mut player_query {
        let speed_scale = 1.0 / (1.0 + player.block_count as f32 * 0.1);
        let rotation_scale = 1.0 / (1.0 + player.block_count as f32 * 0.1);

        let move_speed = PLAYER_CONFIG.movement_speed * speed_scale;
        let rot_speed = PLAYER_CONFIG.rotation_speed * rotation_scale;
        // Handle rotation with A/D keys
        let mut rotation_dir = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            rotation_dir += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            rotation_dir -= 1.0;
        }

        // Handle movement with W/S keys (forward/backward relative to rotation)
        let mut move_dir = bevy::prelude::Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            move_dir.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            move_dir.y -= 1.0;
            //rotation_dir *= -1.;
        }

        // Apply movement relative to player's rotation
        if move_dir != bevy::prelude::Vec3::ZERO {
            //|| rotation_dir != 0. {
            let mut move_direction = bevy::prelude::Vec3::ZERO.clone();
            move_direction = transform.rotation * move_dir.normalize();
            //if move_dir != Vec3::ZERO {
            //}
            let new_pos = transform.translation + move_direction * move_speed * time.delta_secs();

            let collided_with_obstacle = check_collision(
                new_pos.truncate(),
                &obstacle_query,
                PLAYER_CONFIG.size,
                OBSTACLE_CONFIG.size,
            );

            let mut blocks_collided_obstacles = false;

            // copy tanslation ----
            let mut next_frame_pos = transform.clone();
            next_frame_pos.translation = new_pos;
            if rotation_dir != 0.0 {
                next_frame_pos.rotate_z(rotation_dir * rot_speed * time.delta_secs());
            }

            // Check collision for all attached blocks
            for (_attached_block_entity, _attached_block_transform, attached_block_link) in
                attached_block_query.iter()
            {
                if attached_block_link.player_entity == player_entity {
                    let rotated_offset = next_frame_pos.rotation
                        * bevy::prelude::Vec3::new(
                            attached_block_link.grid_offset.0 as f32 * grid.cell_size,
                            attached_block_link.grid_offset.1 as f32 * grid.cell_size,
                            5.0,
                        );

                    let new_block_pos = (next_frame_pos.translation) + (rotated_offset);

                    blocks_collided_obstacles = check_collision(
                        new_block_pos.truncate(),
                        &obstacle_query,
                        PLAYER_CONFIG.size,
                        OBSTACLE_CONFIG.size,
                    );
                    if blocks_collided_obstacles {
                        break;
                    }
                }
            }

            let mut collided_with_block = false;

            // NOTE: Block collision logic here
            let block_radius = BLOCK_CONFIG.size.x.min(BLOCK_CONFIG.size.y) / 2.0;
            let player_radius = PLAYER_CONFIG.size.x.min(PLAYER_CONFIG.size.y) / 2.0;
            let collision_distance = block_radius + player_radius;
            for (_block_entity, block_transform) in block_query.iter_mut() {
                if new_pos
                    .truncate()
                    .distance(block_transform.translation.truncate())
                    < collision_distance
                {
                    collided_with_block = true;
                }
            }

            'outer: for (_block_entity, block_transform, block_link) in attached_block_query.iter()
            {
                if block_link.player_entity != player_entity {
                    if new_pos
                        .truncate()
                        .distance(block_transform.translation.truncate())
                        < collision_distance
                    {
                        collided_with_block = true;
                    }
                    for (_attached_block_entity, _attached_block_transform, attached_block_link) in
                        attached_block_query.iter()
                    {
                        if attached_block_link.player_entity == player_entity {
                            let rotated_offset = next_frame_pos.rotation
                                * bevy::prelude::Vec3::new(
                                    attached_block_link.grid_offset.0 as f32 * grid.cell_size,
                                    attached_block_link.grid_offset.1 as f32 * grid.cell_size,
                                    5.0,
                                );

                            let new_block_pos = (next_frame_pos.translation) + (rotated_offset);

                            if new_block_pos
                                .truncate()
                                .distance(block_transform.translation.truncate())
                                < collision_distance
                            {
                                collided_with_block = true;
                            }
                            if collided_with_block {
                                break 'outer;
                            }
                        }
                    }
                }
            }

            if !collided_with_obstacle
                && !collided_with_block
                && !blocks_collided_obstacles
                && !will_collide(new_pos.truncate(), &obstacle_query)
                && !will_collide_with_opponent(new_pos.truncate(), &opponent_transforms)
            {
                //println!(
                //    "{} {} {} {} {}",
                //    collided_with_obstacle,
                //    collided_with_block,
                //    blocks_collided_obstacles,
                //    will_collide(new_pos.truncate(), &obstacle_query),
                //    will_collide_with_opponent(new_pos.truncate(), &opponent_transforms),
                //);
                // Apply tanslation
                transform.translation = new_pos;
                // Apply rotation
                if rotation_dir != 0.0 {
                    transform.rotate_z(rotation_dir * rot_speed * time.delta_secs());
                }
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

pub fn will_collide(
    new_pos: bevy::prelude::Vec2,
    obstacles: &Query<&Transform, With<Obstacle>>,
) -> bool {
    let player_radius = PLAYER_CONFIG.size.x.min(PLAYER_CONFIG.size.y) / 2.0;
    let obstacle_radius = OBSTACLE_CONFIG.size.x.min(OBSTACLE_CONFIG.size.y) / 2.0;
    let collision_distance = player_radius + obstacle_radius;

    obstacles
        .iter()
        .any(|transform| new_pos.distance(transform.translation.truncate()) < collision_distance)
}

pub fn will_collide_with_opponent(new_pos: bevy::prelude::Vec2, opponents: &[Transform]) -> bool {
    let player_radius = PLAYER_CONFIG.size.x.min(PLAYER_CONFIG.size.y) / 2.0;
    let collision_distance = player_radius * 2.0;

    opponents
        .iter()
        .any(|transform| new_pos.distance(transform.translation.truncate()) < collision_distance)
}

fn generate_random_spawnpoint(ctx_wrapper: &CtxWrapper) -> (f32, f32) {
    let mut rng = rand::rng();
    let mut too_close = false;
    let mut random_x;
    let mut random_y;

    let online_players: Vec<BevyTransform> = ctx_wrapper
        .ctx
        .db
        .player()
        .iter()
        .map(|player| player.position)
        .collect();

    loop {
        random_x = rng
            .random_range(-MAP_CONFIG.safe_zone_size + 50.0..MAP_CONFIG.safe_zone_size - 50.0)
            as f32;
        random_y = rng
            .random_range(-MAP_CONFIG.safe_zone_size + 50.0..MAP_CONFIG.safe_zone_size - 50.0)
            as f32;

        for player_position in &online_players {
            let dx = player_position.coordinates.x - random_x;
            let dy = player_position.coordinates.y - random_y;

            if (dx < PLAYER_CONFIG.size.x && dy < PLAYER_CONFIG.size.y) {
                too_close = true;
                break;
            }
        }
        if too_close {
            continue;
        }
        break;
    }
    (random_x, random_y)
}

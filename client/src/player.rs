use crate::common::{Obstacle, Player, MAP_CONFIG, OBSTACLE_CONFIG, PLAYER_CONFIG};
use crate::db_connection::{update_player_position, CtxWrapper};
use crate::module_bindings::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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
        },
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Player), Without<Obstacle>>,
    obstacle_query: Query<&Transform, With<Obstacle>>,
    time: Res<Time>,
    ctx: Res<CtxWrapper>,
) {
    //if let Ok((mut transform, _player)) = query.get_single_mut() { // NOTE: merge conflict
    let ctx_wrapper = &ctx.into_inner();
    for (mut transform, player) in &mut query {
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
            let move_direction = transform.rotation * move_dir.normalize();
            let new_pos = transform.translation
                + move_direction * PLAYER_CONFIG.movement_speed * time.delta_secs();

            if !will_collide(new_pos.truncate(), &obstacle_query) {
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

fn generate_random_spawnpoint(ctx_wrapper: &CtxWrapper) -> (f32, f32){
    let mut rng = rand::rng();
    let mut too_close = false;
    let mut random_x;
    let mut random_y;

    let online_players: Vec::<BevyTransform> = ctx_wrapper
        .ctx
        .db
        .player()
        .iter()
        .map(|player| player.position)
        .collect();

    loop{
        random_x = rng.random_range(-MAP_CONFIG.safe_zone_size + 50.0..MAP_CONFIG.safe_zone_size - 50.0) as f32;
        random_y = rng.random_range(-MAP_CONFIG.safe_zone_size + 50.0..MAP_CONFIG.safe_zone_size - 50.0) as f32;

        for player_position in &online_players{
            let dx = player_position.coordinates.x - random_x;
            let dy = player_position.coordinates.y - random_y;

            if (dx < PLAYER_CONFIG.size.x && dy < PLAYER_CONFIG.size.y){
                too_close = true;
                break;
            }
        }
        if too_close{
            continue;
        }
        break;
    }
    (random_x, random_y)
}
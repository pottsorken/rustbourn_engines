use crate::db_connection::*;
use crate::module_bindings::*;
use bevy::prelude::*;
use spacetimedb_sdk::{
    credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey,
};

#[derive(Component)]
pub struct Player {
    /// linear speed in meters per second
    pub movement_speed: f32,
    /// rotation speed in radians per second
    pub rotation_speed: f32,
}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a player sprite at position (0, 0) at a higher z-index than map
    commands.spawn((
        Sprite {
            custom_size: Some(bevy::prelude::Vec2::new(80.0, 80.0)), // Square size 100x100 pixels
            image: asset_server.load("sprites/top-view/robot_3Dblue.png"),
            ..default()
        },
        //TextureAtlas {
        //    layout: asset_server.load("sprites/top-view/robot_3Dblue.png"),
        //    index: 0,
        //}, -- NOTE: If asset-chart is ever used
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player {
            movement_speed: 300.0,                  // meters per second
            rotation_speed: f32::to_radians(180.0), // degrees per second
        },
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
    ctx: Res<DbConnection>,
) {
    let ctx_inner = &ctx.into_inner();
    for (mut transform, player) in &mut query {
        // Handle rotation with A/D keys
        let mut rotation_dir = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) {
            rotation_dir += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation_dir -= 1.0;
        }

        // Apply rotation
        if rotation_dir != 0.0 {
            transform.rotate_z(rotation_dir * player.rotation_speed * time.delta_secs());
        }

        // Handle movement with W/S keys (forward/backward relative to rotation)
        let mut move_dir = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            move_dir.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            move_dir.y -= 1.0;
        }

        // Apply movement relative to player's rotation
        if move_dir != Vec3::ZERO {
            let move_direction = transform.rotation * move_dir.normalize();
            transform.translation += move_direction * player.movement_speed * time.delta_secs();
        }
        update_player_position(ctx_inner, &transform);
    }
}

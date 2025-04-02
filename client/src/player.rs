use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const PLAYER_SIZE: f32 = 100.0;

#[derive(Component)]
pub struct Player {
    /// linear speed in meters per second
    pub movement_speed: f32,
    /// rotation speed in radians per second
    pub rotation_speed: f32,
}

pub fn setup_player(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    // Spawn a player sprite at position (0, 0) at a higher z-index than map
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(80.0, 80.0)), // Square size 100x100 pixels
            image: asset_server.load("sprites/top-view/robot_3Dblue.png"),
            ..default()
        },
        //TextureAtlas {
        //    layout: asset_server.load("sprites/top-view/robot_3Dblue.png"),
        //    index: 0,
        //}, -- NOTE: If asset-chart is ever used
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player {
            movement_speed: 3000.0,                  // meters per second
            rotation_speed: f32::to_radians(180.0), // degrees per second
        },
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    if let Ok((mut transform, player)) = query.get_single_mut() {
        // Handle rotation with A/D keys
        let mut rotation_dir = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft){
            rotation_dir += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight){
            rotation_dir -= 1.0;
        }

        // Apply rotation
        if rotation_dir != 0.0 {
            transform.rotate_z(rotation_dir * player.rotation_speed * time.delta_secs());
        }

        // Handle movement with W/S keys (forward/backward relative to rotation)
        let mut move_dir = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp){
            move_dir.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown){
            move_dir.y -= 1.0;
        }

        // Apply movement relative to player's rotation
        if move_dir != Vec3::ZERO {
            let move_direction = transform.rotation * move_dir.normalize();
            transform.translation += move_direction * player.movement_speed * time.delta_secs();
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size: f32 = PLAYER_SIZE / 2.0;
        let x_min: f32 = -8192.0/2.0 + half_player_size;
        let x_max: f32 = 8192.0/2.0 - half_player_size;
        let y_min: f32 = -8192.0/2.0 + half_player_size;
        let y_max: f32 = 8192.0/2.0 - half_player_size;

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
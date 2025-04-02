use bevy::prelude::*;
use crate::player::Player;
use crate::player_attach::PlayerAttach;

#[derive(Component)]
pub struct Hook {
    pub hook_speed: f32,
    pub hook_max_range: f32,
}

pub fn setup_hook(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a player sprite at position (0, 0) at a higher z-index than map
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(12.0, 26.0)), // Square size 100x100 pixels
            // image: asset_server.load("C:\\Users\\Denise\\Desktop\\Studier\\CINTE\\II1305 Projektkurs\\rustbourn_engines\\client\\assets\\sprites\\top-view\\robot_yellow.png"),
            image: asset_server.load("sprites/top-view/robot_yellow.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 5.0),
        Hook {
            hook_speed: 500.0,
            hook_max_range: 100.0,
        },
        PlayerAttach {
            offset: Vec2::new(0.0, 20.0), // Offset from player's center
        },
    ));
}

pub fn hook_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut query {

        // Handle movement with W/S keys (forward/backward relative to rotation)
        let mut move_dir = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Space) {
            move_dir.y += 1.0;
        }

        // Apply movement relative to player's rotation
        if move_dir != Vec3::ZERO {
            let move_direction = transform.rotation * move_dir.normalize();
            transform.translation += move_direction * player.movement_speed * time.delta_secs();
        }
    }
}
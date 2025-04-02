use bevy::prelude::*;

use crate::player::{Player,
    setup_player,
    player_movement};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 999.9),
    ));
}


pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
) {
    let follow_speed = 3.0;

    if let Ok(player_transform) = player_query.get_single() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation = camera_transform.translation.lerp(
                Vec3::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                    camera_transform.translation.z, // Ensure camera z-pos unchanged
                ),
                follow_speed * time.delta_secs(),
            );
        }
    }
}
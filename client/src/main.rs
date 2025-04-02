use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noisy_bevy::simplex_noise_2d; // For map generation. May be temporary

mod player;
mod map;

use player::{Player,
            setup_player,
            player_movement};

use map::{setup_tilemap};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window : Some(Window{
                    title : String::from("Rustbourn Engines"),
                    position : WindowPosition::Centered(MonitorSelection::Primary),
                    ..Default::default()
                }),
                ..Default::default()
            })
        )
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, (setup_camera, setup_player, setup_tilemap))
        .add_systems(Update, (player_movement, camera_follow))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 999.9),
        ..default()
    });
}


fn camera_follow(
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

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noisy_bevy::simplex_noise_2d; // For map generation. May be temporary

mod player;
mod map;
mod camera;

use player::{Player,
            setup_player,
            player_movement};

use map::{setup_tilemap};

use camera::{setup_camera,
            camera_follow};

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

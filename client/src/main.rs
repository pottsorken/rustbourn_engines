// Game engine
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noisy_bevy::simplex_noise_2d; // For map generation. May be temporary

mod camera;
mod db_connection;
mod map;
mod module_bindings;
mod parse;
mod player;

use camera::{camera_follow, setup_camera};
use db_connection::setup_connection;
use map::setup_tilemap;
use parse::*;
use player::{player_movement, setup_player, Player};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Rustbourn Engines"),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(TilemapPlugin)
        .add_systems(
            Startup,
            (setup_camera, setup_player, setup_tilemap, setup_connection),
        )
        .add_systems(Update, (player_movement, camera_follow))
        .run();
}

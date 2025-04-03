// Game engine
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noisy_bevy::simplex_noise_2d; // For map generation. May be temporary

mod camera;
mod common;
mod db_connection;
mod map;
mod obstacle;
mod player;

use camera::*;
use map::*;
use obstacle::*;
use player::*;

//#[cfg(windows)]
//#[global_allocator]
//static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// Spacedime dependencies
mod module_bindings;
mod opponent;
mod parse;
mod player;

use camera::{camera_follow, setup_camera};
use db_connection::{print_player_positions, setup_connection};
use map::setup_tilemap;
use opponent::despawn_opponents;
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
            (
                setup_camera,
                setup_player,
                setup_tilemap,
                setup_connection,
                setup_obstacle,
            ),
        )
        .add_systems(
            Update,
            (
                player_movement,
                confine_player_movement,
                camera_follow,
                print_player_positions,
                despawn_opponents,
            ),
        )
        .run();
}

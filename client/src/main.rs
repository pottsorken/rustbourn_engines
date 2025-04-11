// Game engine
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
//use noisy_bevy::simplex_noise_2d; // For map generation. May be temporary

mod block;
mod bots;
mod camera;
mod common;
mod db_connection;
mod grid;
mod hook;
mod map;
mod module_bindings;
mod obstacle;
mod opponent;
mod parse;
mod player;
mod player_attach;

use block::*;
use hook::*;
use obstacle::*;
use player::*;
use player_attach::*;

//#[cfg(windows)]
//#[global_allocator]
//static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
// Spacedime dependencies

use bots::{spawn_bot_blocks, spawn_bots, update_bots};
use camera::{camera_follow, setup_camera};
use db_connection::{print_player_positions, setup_connection};
use map::setup_tilemap;
use opponent::despawn_opponents;
use player::{player_movement, setup_player};

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
                setup_connection,
                setup_camera,
                setup_player,
                setup_tilemap,
                setup_block,
                setup_hook,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                player_movement,
                update_block,
                confine_player_movement,
                camera_follow,
                print_player_positions,
                hook_collision_system,
                hook_controls,
                
                attach_objects,
                attach_items,
                attach_block,
                update_bots,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                setup_obstacle,
                despawn_opponents,
                spawn_bots,
                spawn_bot_blocks,
            ),
        )
        .insert_resource(Time::from_seconds(0.5))
        .run();
}

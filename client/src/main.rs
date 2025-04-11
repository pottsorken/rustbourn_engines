// Game engine
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noisy_bevy::simplex_noise_2d; // For map generation. May be temporary

mod camera;
mod common;
mod db_connection;
mod hook;
mod map;
mod module_bindings;
mod obstacle;
mod opponent;
mod parse;
mod player;
mod player_attach;
mod bots;

use camera::*;
use hook::*;
use map::*;
use obstacle::*;
use player::*;
use player_attach::*;
use bots::*;

//#[cfg(windows)]
//#[global_allocator]
//static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
// Spacedime dependencies

use camera::{camera_follow, setup_camera};
use db_connection::{print_player_positions, setup_connection, update_opponent_hooks};
use map::setup_tilemap;
use opponent::despawn_opponents;
use parse::*;
use player::{player_movement, setup_player};
use bots::{spawn_bots, render_bots_from_db};
use hook::handle_obstacle_hit;

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
                setup_hook,
            ).chain(), // So the startup happens in order.
        )
        .add_systems(
            Update,
            (
                player_movement,
                confine_player_movement,
                camera_follow,
                print_player_positions,
                hook_controls,
                attatch_objects,
                render_bots_from_db,
                update_opponent_hooks,
                handle_obstacle_hit,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                setup_obstacle,
                despawn_opponents,
                spawn_bots,
                
                
            )
        )
        .insert_resource(Time::from_seconds(0.5))
        .insert_resource(SpawnedObstacles::default())
        .run();
}



// Game engine
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noisy_bevy::simplex_noise_2d; // For map generation. May be temporary
use bevy::{app::AppExit, color::palettes::css::CRIMSON, prelude::*};

mod block;
mod bots;
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
mod start_menu;

use block::*;
use bots::*;
use camera::*;
use hook::*;
use map::*;
use obstacle::*;
use player::*;
use player_attach::*;
use start_menu::*;

//#[cfg(windows)]
//#[global_allocator]
//static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
// Spacedime dependencies

use bots::{spawn_bots, update_bots};
use camera::{camera_follow, setup_camera};
use db_connection::{print_player_positions, setup_connection};
use map::setup_tilemap;
use opponent::despawn_opponents;
use parse::*;
use player::{player_movement, setup_player};
use common::*;


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
        .init_state::<GameState>()
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .add_plugins((splash_plugin, menu_plugin, game_plugin))
        .add_systems(
            Startup,
            (
                setup_camera,
            )
            .chain(),
        )
        .add_systems(
            OnEnter(GameState::Game),
            (
                setup_connection,
                setup_player,
                setup_tilemap,
                setup_block,
                setup_hook,
            )
                .chain()
        )
        .add_systems(
            Update,
            (
                player_movement,
                update_block,
                confine_player_movement,
                camera_follow,
                print_player_positions,
                hook_controls,
                attach_objects,
                update_bots,
            )
                .run_if(in_state(GameState::Game))
        )
        .add_systems(
            FixedUpdate,
            (setup_obstacle, despawn_opponents, spawn_bots)
                .run_if(in_state(GameState::Game))
        )
        .insert_resource(Time::from_seconds(0.5))
        .run();
}
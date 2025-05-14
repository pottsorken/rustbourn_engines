// Game engine
use bevy::{prelude::*, ui::update};
use bevy_ecs_tilemap::prelude::*;
use noisy_bevy::simplex_noise_2d; // For map generation. May be temporary
use bevy::{app::AppExit, color::palettes::css::CRIMSON, prelude::*};

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
mod start_menu;
mod edit_menu; use edit_menu::*;
mod track_spawner;

use track_spawner::*;
use block::*;
use camera::*;
use hook::*;
use obstacle::*;
use player::*;
use player_attach::*;
use start_menu::*;
use common::*;

//#[cfg(windows)]
//#[global_allocator]
//static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
// Spacedime dependencies

use bots::{render_bots_from_db, spawn_bot_blocks, spawn_bots};
use camera::{camera_follow, setup_camera};
use db_connection::{update_opponent_positions, setup_connection, update_opponent_hooks, update_opponent_tracks, db_setup};
use grid::{check_grid_connectivity,  balance_player_grid, balance_opponents_grid};
use hook::handle_obstacle_hit;
use map::setup_tilemap;
use opponent::{despawn_opponents, spawn_opponent_tracks_system, setup_blocks_opponent};
use common::*;
use track_spawner::{spawn_tracks_system, track_lifetime_system};
use player::{player_movement, setup_blocks_player, setup_player};

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
        .add_plugins((splash_plugin, menu_plugin, game_plugin, edit_plugin))
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
                camera_zoom,
                update_opponent_positions,
                hook_collision_system,
                hook_controls,
                handle_obstacle_hit,
                track_lifetime_system,
                render_bots_from_db,
                attach_objects,
                attach_items,
                update_opponent_hooks,
                spawn_tracks_system,
                track_lifetime_system,
                spawn_opponent_tracks_system,
                update_opponent_tracks, 
                handle_obstacle_hit,
                check_grid_connectivity,
            ) 
            .run_if(in_game_or_edit),
        )
        .add_systems(
            Update,
             (
                update_block_owner,
                balance_player_grid,
                balance_opponents_grid,
             )
             .run_if(in_game_or_edit),
            )
        .add_systems(
            FixedUpdate,
            (
                setup_obstacle,
                despawn_opponents,
                spawn_bots,
                setup_blocks_player,
                spawn_bot_blocks,
                setup_blocks_opponent,
            )
            .run_if(in_game_or_edit),
        )
        .insert_resource(Time::from_seconds(0.5))
        .insert_resource(SpawnedObstacles::default())
        .insert_resource(CtxWrapper {
            ctx: db_setup(),
        })
        .insert_resource(HookTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .run();
}
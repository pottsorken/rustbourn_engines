// Game engine
use bevy::{app::AppExit, color::palettes::css::CRIMSON, prelude::*};
use bevy::{prelude::*, ui::update};
use bevy_ecs_tilemap::prelude::*;
use noisy_bevy::simplex_noise_2d; // For map generation. May be temporary

mod block;
mod bots;
mod camera;
mod common;
mod db_connection;
mod edit_menu;
mod grid;
mod hook;
mod leaderboard;
mod map;
mod module_bindings;
mod nametag;
mod obstacle;
mod opponent;
mod parse;
mod player;
mod player_attach;
mod start_menu;
mod track_spawner;

use block::*;
use camera::*;
use common::*;
use edit_menu::*;
use hook::*;
use leaderboard::*;
use leaderboard::*;
use nametag::*;
use obstacle::*;
use player::*;
use player_attach::*;
use start_menu::*;
use track_spawner::*;

//#[cfg(windows)]
//#[global_allocator]
//static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
// Spacedime dependencies

use bots::{render_bots_from_db, send_bots_to_db, spawn_bot_blocks, spawn_bots};
use camera::{camera_follow, setup_camera};
use common::*;
use db_connection::{
    db_setup, setup_connection, update_opponent_hooks, despawn_opponent_hooks, update_opponent_positions,
    update_opponent_tracks,
};
use grid::{balance_opponents_grid, balance_player_grid, check_grid_connectivity};

use hook::{handle_obstacle_hit, hook_cooldown_system};
use leaderboard::{spawn_leaderboard, update_leaderboard_from_db};
use map::setup_tilemap;
use opponent::{despawn_opponents, setup_blocks_opponent, spawn_opponent_tracks_system};
use player::{player_movement, setup_blocks_player, setup_player};
use track_spawner::{spawn_tracks_system, track_lifetime_system};

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
        //.insert_resource(Leaderboard::default())
        .insert_resource(Volume(7))
        .add_plugins((splash_plugin, menu_plugin, game_plugin)) //edit_plugin
        .add_systems(Startup, (setup_camera,).chain())
        .add_systems(
            OnEnter(GameState::Game),
            (
                setup_connection,
                setup_player,
                setup_tilemap,
                setup_hook,
                spawn_tags,
                spawn_leaderboard,
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
                camera_zoom,
                update_opponent_positions,
                hook_collision_system,
                hook_controls,
                handle_obstacle_hit,
                track_lifetime_system,
                attach_objects,
                attach_items,
                update_opponent_hooks,
                spawn_tracks_system,
                spawn_opponent_tracks_system,
                update_opponent_tracks,
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
                hook_cooldown_system,
                despawn_opponent_hooks,
                send_bots_to_db,
                render_bots_from_db,
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
                update_nametags_content, // update_bots,
                update_leaderboard_from_db,
            )
                .run_if(in_game_or_edit),
        )
        .insert_resource(Time::from_seconds(0.5))
        .insert_resource(SpawnedObstacles::default())
        .insert_resource(CtxWrapper { ctx: db_setup() })
        .insert_resource(HookTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .insert_resource(SpawnedBlocks::default())
        .run();
}

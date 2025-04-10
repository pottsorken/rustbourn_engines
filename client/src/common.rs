//
// Configuration and shared components for the game
//

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use spacetimedb_sdk::Identity;
use std::collections::HashMap;

//
// === Bot defined constraints ===
//

// Component to mark a bot and track its state
#[derive(Component)]
pub struct Bot {
    pub id: u64,
    pub spawn_point: Vec2,
    pub movement_speed: f32,
}

// Configuration for spawning bots
pub struct BotConfig {
    pub size: Vec2,
    pub path: &'static str,
    pub count: usize,
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

// Global bot config
pub const BOT_CONFIG: BotConfig = BotConfig {
    size: Vec2::new(80.0, 80.0),
    path: "sprites/top-view/robot_3Dyellow.png",
    count: 3,
    movement_speed: 200.0,
    rotation_speed: f32::to_radians(180.0),
};

//
// === Player defined constraints ===
//

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub block_count: i32,
}

#[derive(Component)]
pub struct PlayerAttach {
    pub offset: Vec2,
}

#[derive(Component)]
pub struct PlayerGrid {
    pub block_position: HashMap<(i32, i32), Entity>,
    pub grid_size: (i32, i32),
    pub cell_size: f32,
    pub next_free_pos: (i32, i32),
    pub capacity: u32,
    pub load: u32,
}

#[derive(Component)]
pub struct AttachedBlock {
    pub grid_offset: (i32, i32),
    pub player_entity: Entity,
}
// Configuration struct for initializing the Player entity
#[derive(Component)]
pub struct PlayerConfig {
    pub size: Vec2,
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub path: &'static str,
    pub max_block_count: i32,
}
// Global constant config for the player
#[derive(Component)]
pub struct OpponentHook {
    pub id: Identity, // Match with the opponent's identity
}
/// Global constant config for the player
pub const PLAYER_CONFIG: PlayerConfig = PlayerConfig {
    size: Vec2::new(80.0, 80.0),
    movement_speed: 300.0,
    rotation_speed: f32::to_radians(180.0),
    path: "sprites/top-view/robot_3Dblue.png",
    max_block_count: 100,
};

//
// === Grid defined constraints ===
//
#[derive(Component)]
pub struct GridConfig {
    pub grid_size: (i32, i32),
    pub cell_size: f32,
    pub next_free_pos: (i32, i32),
    pub capacity: u32,
    pub load: u32,
}
// Global constant config for the player
pub const GRID_CONFIG: GridConfig = GridConfig {
    grid_size: (1, 1),
    cell_size: 84.,
    next_free_pos: (-1, 0),
    capacity: 100,
    load: 0,
};

//
// === Camera defined constraints ===
//
#[derive(Component)]
pub struct CameraConfig {
    pub zoom_base: f32,
    pub zoom_per_blocks: f32,
    pub zoom_after_blocks: i32,
}
// Global constant config for the player
pub const CAMERA_CONFIG: CameraConfig = CameraConfig {
    zoom_base: 1.0,
    zoom_per_blocks: 0.2,
    zoom_after_blocks: 5,
};

//
// === Opps defined constraints ===
//
#[derive(Component)]
pub struct Opponent {
    /// linear speed in meters per second
    pub movement_speed: f32,
    /// rotation speed in radians per second
    pub rotation_speed: f32,

    // DB identity
    pub id: Identity,
}

//
// === Hook defined constraints ===
//

#[derive(Component)]
pub struct Hook {
    pub hook_speed: f32,
    pub hook_max_range: f32,
}

#[derive(Component)]
pub struct HookCharge {
    pub time_held: f32,
    pub target_length: f32,
}

#[derive(Component)]
pub struct HookConfig {
    pub hook_size: Vec2,
    pub hook_path: &'static str,
    pub hook_speed: f32,
    pub hook_max_range: f32,
    pub extend_speed: f32,
    pub retract_speed: f32,
    pub hook_radius: f32,
    pub player_attach_offset: Vec2,
}

pub const HOOK_CONFIG: HookConfig = HookConfig {
    hook_size: Vec2::new(25.0, 0.0),
    hook_path: "sprites/GrapplingHook.png",
    hook_speed: 500.0,
    hook_max_range: 400.0,
    extend_speed: 500.0,
    retract_speed: 500.0, // Could use hook_speed here too
    hook_radius: 5.0,
    player_attach_offset: Vec2::new(0.0, 20.0),
};

//
// === Block defined constraints ===
//

#[derive(Component)]
pub struct Block {}
pub struct BlockConfig {
    pub size: Vec2,
    pub rotation_speed: f32,
    pub path: &'static str,
    pub count: i32,
}
/// Global constant config for the block
pub const BLOCK_CONFIG: BlockConfig = BlockConfig {
    size: Vec2::new(80.0, 80.0),
    rotation_speed: f32::to_radians(90.0),
    path: "sprites/top-view/robot_green.png",
    count: 100,
};

//
// === Obstacle defined constraints ===
//

#[derive(Component)]
pub struct Obstacle {
    pub id: u64,
}

// Configuration struct for initializing the Player entity
pub struct ObstacleConfig {
    pub size: Vec2,
    pub count: i32,
    pub path: &'static str,
}

// Global constant config for the player
pub const OBSTACLE_CONFIG: ObstacleConfig = ObstacleConfig {
    size: Vec2::new(80.0, 80.0),
    count: 1000,
    path: "sprites/Obstacles/obstacle_rock.png",
};

//
// === Map related definitions ===
//

/// Configuration for initlializing the tilemap
pub struct MapConfig {
    pub map_size: TilemapSize,
    pub tile_size: TilemapTileSize, // tiles are 16x16 pixels
    pub noise_scale: f32,           // Grid size == tile size
    pub tile_textures: [&'static str; 4],
    pub image_path: &'static str,
    pub safe_zone_size: f32,
}

/// Global constant config for the tilemap
pub const MAP_CONFIG: MapConfig = MapConfig {
    map_size: TilemapSize { x: 64, y: 64 },
    tile_size: TilemapTileSize { x: 128.0, y: 128.0 }, // tiles are 16x16 pixels
    noise_scale: 0.1,
    tile_textures: [
        "sprites/td_tanks/grass.png",
        "sprites/td_tanks/water.png",
        "sprites/td_tanks/stone.png",
        "sprites/td_tanks/dirt.png",
    ],
    image_path: r"assets/tribasicmap64.png",
    safe_zone_size: 300.0,
};

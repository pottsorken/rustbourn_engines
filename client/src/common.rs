use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

/// Configuration and shared components for the game
/// ------------------------------------------------


/// Component to mark a bot and track its state
#[derive(Component)]
pub struct Bot {
    pub id: u64,
    pub spawn_point: Vec2,
    pub movement_speed: f32,
}

/// Configuration for spawning bots
pub struct BotConfig {
    pub size: Vec2,
    pub path: &'static str,
    pub count: usize,
    pub movement_speed: f32,
    pub rotation_speed: f32,

}

/// Global bot config
pub const BOT_CONFIG: BotConfig = BotConfig {
    size: Vec2::new(80.0, 80.0),
    path: "sprites/top-view/robot_3Dyellow.png",
    count: 3,
    movement_speed: 200.0,
    rotation_speed: f32::to_radians(180.0),

};

// === Player defined constraints ===

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

/// Configuration struct for initializing the Player entity
#[derive(Component)]
pub struct PlayerConfig {
    pub size: Vec2,
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub path: &'static str,
}
#[derive(Component)]
pub struct Hook {
    pub hook_speed: f32,
    pub hook_max_range: f32,
}
#[derive(Component)]
pub struct PlayerAttach {
    pub offset: Vec2,
}
/// Global constant config for the player
pub const PLAYER_CONFIG: PlayerConfig = PlayerConfig {
    size: Vec2::new(80.0, 80.0),
    movement_speed: 300.0,
    rotation_speed: f32::to_radians(180.0),
    path: "sprites/top-view/robot_3Dblue.png",
};

// === Block defined constraints ===

#[derive(Component)]
pub struct Block{
    
}
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


// === Obstacle defined constraints ===

#[derive(Component)]
pub struct Obstacle {
    pub id: u64,
}

/// Configuration struct for initializing the Player entity
pub struct ObstacleConfig {
    pub size: Vec2,
    pub count: i32,
    pub path: &'static str,
}

/// Global constant config for the player
pub const OBSTACLE_CONFIG: ObstacleConfig = ObstacleConfig {
    size: Vec2::new(80.0, 80.0),
    count: 1000,
    path: "sprites/Obstacles/obstacle_rock.png",
};

// === Map related definitions ===

/// Configuration for initlializing the tilemap
pub struct MapConfig {
    pub map_size: TilemapSize,
    pub tile_size: TilemapTileSize, // tiles are 16x16 pixels
    pub noise_scale: f32,           // Grid size == tile size
    pub tile_textures: [&'static str; 3],
    pub safe_zone_size: f32,
}

/// Global constant config for the tilemap
pub const MAP_CONFIG: MapConfig = MapConfig {
    map_size: TilemapSize { x: 64, y: 64 },
    tile_size: TilemapTileSize { x: 128.0, y: 128.0 }, // tiles are 16x16 pixels
    noise_scale: 0.1,
    tile_textures: [
        "sprites/td_tanks/dirt.png",
        "sprites/td_tanks/stone.png",
        "sprites/td_tanks/grass.png",
    ],
    safe_zone_size: 300.0,
};

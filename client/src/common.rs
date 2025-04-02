use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

/// Configuration and shared components for the game
/// ------------------------------------------------

// === Player defined constraints ===

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

/// Configuration struct for initializing the Player entity
pub struct PlayerConfig {
    pub size: Vec2,
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub sprite_path: &'static str,
}

/// Global constant config for the player
pub const PLAYER_CONFIG: PlayerConfig = PlayerConfig {
    size: Vec2::new(80.0, 80.0),
    movement_speed: 1000.0,
    rotation_speed: f32::to_radians(180.0),
    sprite_path: "sprites/top-view/robot_3Dblue.png",
};

// === Map related definitions ===

/// Configuration for initlializing the tilemap
pub struct MapConfig {
    pub map_size: TilemapSize,
    pub tile_size: TilemapTileSize, // tiles are 16x16 pixels
    pub noise_scale: f32, // Grid size == tile size
    pub tile_textures: [&'static str; 3],
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
    ]
};

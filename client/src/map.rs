use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use image::{GenericImageView, ImageReader};
use std::collections::HashSet;
use crate::common::{MAP_CONFIG, WaterTiles, DirtTiles, GrassTiles, StoneTiles};

// Define the Obstacle component
#[derive(Component)]
pub struct Obstacle;

// Define the Obstacle component
#[derive(Component)]
pub struct Modifier;

pub fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load tile textures
    let texture_handle: Vec<Handle<Image>> = MAP_CONFIG
        .tile_textures
        .iter()
        .map(|path| asset_server.load(*path))
        .collect();

    let grid_size = MAP_CONFIG.tile_size.into();
    let map_type = TilemapType::default();
    let mut tile_storage = TileStorage::empty(MAP_CONFIG.map_size);
    let tilemap_entity = commands.spawn_empty().id();

    // Load the image
    let img = ImageReader::open(MAP_CONFIG.image_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    let (width, height) = img.dimensions();
    let mut water_tiles = HashSet::new();
    let mut dirt_tiles = HashSet::new();
    let mut grass_tiles = HashSet::new();
    let mut stone_tiles = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            let tile_pos = TilePos { x, y };
            let pixel = img.get_pixel(x, y);

            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            let texture_index = if g == 255 {
                grass_tiles.insert((x, y)); // Track grass tiles
                0 // Grass
            } else if b == 255 {
                water_tiles.insert((x, y)); // Track water tiles
                1 // Water
            } else if r == 255 {
                stone_tiles.insert((x, y)); // Track stone tiles
                2 // Stone
            } else {
                dirt_tiles.insert((x, y)); // Track dirt tiles
                3 // Dirt (default)
            };

            let tile_bundle = TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(texture_index),
                ..Default::default()
            };

            let tile_entity = if texture_index == 1 {
                // For water tiles, add the Obstacle component
                commands.spawn(tile_bundle)
                    .insert(Obstacle) // Add the Obstacle component for water tiles
                    .id()
            } else if texture_index == 2 {
                // For stone tiles, add the Modifier component
                commands.spawn(tile_bundle)
                    .insert(Modifier) // Add the Modifier component for stone tiles
                    .id()
            } else if texture_index == 0 {
                // For grass tiles, add the GrassTiles component
                commands.spawn(tile_bundle)
                    .insert(Modifier) // Add the GrassTiles component for grass tiles
                    .id()
            } else if texture_index == 3 {
                // For dirt tiles, add the DirtTiles component
                commands.spawn(tile_bundle)
                    .insert(Modifier) // Add the DirtTiles component for dirt tiles
                    .id()
            } else {
                // For other tiles, spawn without Obstacle
                commands.spawn(tile_bundle)
                    .id()
            };

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // Insert tilemap entity
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: MAP_CONFIG.map_size,
        storage: tile_storage,
        texture: TilemapTexture::Vector(texture_handle),
        tile_size: MAP_CONFIG.tile_size,
        transform: get_tilemap_center_transform(&MAP_CONFIG.map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    // Insert water tile data as a resource
    commands.insert_resource(WaterTiles {
        positions: water_tiles,
    });

    // Insert water tile data as a resource
    commands.insert_resource(DirtTiles {
        positions: dirt_tiles,
    });

    // Insert grass tile data as a resource
    commands.insert_resource(GrassTiles {
        positions: grass_tiles,
    });

    // Insert stone tile data as a resource
    commands.insert_resource(StoneTiles {
        positions: stone_tiles,
    });
}

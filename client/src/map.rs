use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noisy_bevy::simplex_noise_2d;
use crate::common::MAP_CONFIG;

pub fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Vec<Handle<Image>> = MAP_CONFIG
        .tile_textures
        .iter()
        .map(|path|asset_server.load(*path))
        .collect();

    // New map with 64x64 chunks being 32x32 tiles
    let map_size = MAP_CONFIG.map_size;
    let tile_size = MAP_CONFIG.tile_size; // tiles are 16x16 pixels
    let grid_size = tile_size.into(); // Grid size == tile size
    let map_type = TilemapType::default();

    // New tile storage
    let mut tile_storage = TileStorage::empty(map_size);

    // spawn entity
    let tilemap_entity = commands.spawn_empty().id();

    let noise_scale = 0.1;

    // Fill the tilemap with some tiles
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };

            // Determine tile texture
            let noise_value = simplex_noise_2d(Vec2::new(
                x as f32 * MAP_CONFIG.noise_scale, 
                y as f32 * MAP_CONFIG.noise_scale,
            ));

            let texture_index = if noise_value > 0.5 {
                2 // grass
            } else if noise_value > 0.0 {
                1 // stone
            } else {
                0 // dirt
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(texture_index), // first tile in tileset
                    ..Default::default()
                })
                .id();
            
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Vector(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
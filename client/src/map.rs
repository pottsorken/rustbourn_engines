use crate::common::MAP_CONFIG;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use image::{io::Reader as ImageReader, GenericImageView};
use noisy_bevy::simplex_noise_2d;

pub fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Tile images. ORDER IS IMPORTANT!
    let texture_handle: Vec<Handle<Image>> = vec![
        asset_server.load("sprites/td_tanks/grass.png"),
        asset_server.load("sprites/td_tanks/water.png"),
        asset_server.load("sprites/td_tanks/stone.png"),
        asset_server.load("sprites/td_tanks/dirt.png"),
    ];
    // New map with 64x64 chunks being 32x32 tiles
    let map_size = TilemapSize { x: 64, y: 64 };
    let tile_size = TilemapTileSize { x: 128.0, y: 128.0 }; // tiles are 16x16 pixels
    let grid_size = tile_size.into(); // Grid size == tile size
    let map_type = TilemapType::default();

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let image_path = r"assets/tribasicmap64.png";

    // Load the image
    let img = ImageReader::open(image_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let tile_pos = TilePos { x, y };

            let pixel = img.get_pixel(x, y);

            // Split pixel colors into RGB
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            let texture_index = if g == 255 {
                0 // Green -> Grass
            } else if b == 255 {
                1 // Blue -> Water
            } else if r == 255 {
                2 // Red -> Stone
            } else {
                3 // Default -> Dirt
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(texture_index),
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


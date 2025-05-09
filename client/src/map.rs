//use noisy_bevy::simplex_noise_2d;
use crate::common::MAP_CONFIG;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use image::{GenericImageView, ImageReader};
// use rand::random;
use rand::Rng;

pub fn setup_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut image: ResMut<Assets<Image>>,
) {
    // Tile images. ORDER IS IMPORTANT!
    let texture_handle: Vec<Handle<Image>> = MAP_CONFIG
        .tile_textures
        .iter()
        .map(|path| {
            let handle = asset_server.load(*path);
            handle
        })
        .collect();

    // New map with 64x64 chunks being 32x32 tiles
    let grid_size = MAP_CONFIG.tile_size.into(); // Grid size == tile size
    let map_type = TilemapType::default();

    let mut tile_storage = TileStorage::empty(MAP_CONFIG.map_size);
    let tilemap_entity = commands.spawn_empty().id();

    // Load the image
    let img = ImageReader::open(MAP_CONFIG.image_path)
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

            // let mut start;
            // let mut stop;

            // let texture_index = if g == 255 {
            //     0 // Green -> Grass
            // } else if b == 255 {
            //     1 // Blue -> Water
            // } else if r == 255 {
            //     2 // Red -> Stone
            // } else {
            //     3 // Default -> Dirt
            // };

            let mut rng = rand::thread_rng();
            let texture_index = if g >= 220 {
                rng.gen_range(0..20)
                // 0 // Green -> Grass
            } else if >= 220 {
                rng.gen_range(30..39)
                // 2 // Blue -> Water
            } else if >= 220 {
                3 // Red -> Stone
            } else {
                4 // Default -> Dirt
            };

            // if g == 255 {
            //     start = 0;
            //     stop = 16;
            //     texture
            // }

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
        size: MAP_CONFIG.map_size,
        storage: tile_storage,
        texture: TilemapTexture::Vector(texture_handle),
        tile_size: MAP_CONFIG.tile_size,
        transform: get_tilemap_center_transform(&MAP_CONFIG.map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

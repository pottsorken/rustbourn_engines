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

            // GRASS
            let texture_index = if r <= 20 && g >= 230 && b <= 20 {
                rng.gen_range(0..20)
            // WATER
            } else if r <= 20 && g <= 20 && b >= 230  {
                rng.gen_range(20..29)
            // STONE
            } else if r <= 20 && g <= 20 && b <= 20  {
                rng.gen_range(29..38)
            // DIRT
            } else if r >= 230 && g >= 230 && b >= 230  {
                rng.gen_range(38..48)
            // LAVA 
            } else if r >= 230 && g <= 20 && b <= 20  {
                rng.gen_range(48..58)
            // WATER-GRASS
            } else if r <= 20 && g >= 230 && b >= 230  {
                rng.gen_range(58..62)
            // WATER-STONE
            } else if r >= 230 && g <= 20 && b >= 230 {
                rng.gen_range(62..67)
            // DIRT-GRASS
            } else if r >= 230 && g >= 230 && b <= 20  {
                rng.gen_range(67..77)
            // DIRT-STONE
            } else if r <= 200 && r >= 100 && g <= 200 && g >= 100 && b <= 200 && b >= 100  {
                rng.gen_range(77..81)
            // STONE-GRASS
            } else if r <= 70 && r >= 30 && g <= 70 && g >= 30 && b <= 70 && b >= 30  {
                rng.gen_range(81..86)
            } else {
                49
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
